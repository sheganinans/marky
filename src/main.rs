#[macro_use] extern crate clap;

use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use indicatif::ProgressBar;
use markov::Chain;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de::DeserializeOwned};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct   HL2 {                   p: F, v: u64 }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct  OHLC { o: F, h: F, l: F, c: F }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct OHLCV { o: F, h: F, l: F, c: F, v: u64 }

fn main() {
    let matches = clap_app!(marky =>
        (version: "0.0.6")
        (author: "Aistis Raulinaitis. <sheganians@gmail.com>")
        (about: "marky, the CSV time series MCMC trainer")
        (@arg DESIRED_LEN: +required "desired length of history")
        (@arg INPUT: +required "input file")
        (@arg OUTPUT: -o --output +takes_value "output destination")
        (@arg CHUNKING: -c --chunking +takes_value "chunking factor (default 10)")
        (@arg CHUNK_DELTA: -t --delta +takes_value "chunking delta (default φ)")
        (@arg NUM_FILES: -n --num +takes_value "generate n mumber of files named `n.out.csv`")
        (@arg DIVISOR: -r --divisor +takes_value "set `max(len(chunks)) < len(rows)/divisor` (useful for large files)")
        (@arg SILENT: -s --silent "make me shut up")
        (@arg HEADER: --header "has header (default false)")
        (@arg ORDER: -d ... "increase order of MCMC")
        (@arg HL2: --hl2 "HL2 mode")
        (@arg OHLC: --ohlc "OHLC mode")
        (@arg OHLCV: --ohlcv "OHLCV mode")
        (@arg F64: --f64 "f64 mode (default true)")
        (@arg I64: --i64 "i64 mode")
        (@arg U64: --u64 "u64 mode")
    ).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let desired_len = matches.value_of("DESIRED_LEN").unwrap().parse().unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("out.csv");
    let chunking = matches.value_of("CHUNKING").unwrap_or("10").parse().unwrap();
    let chunk_delta : f64 = matches.value_of("CHUNK_DELTA").unwrap_or("1.618033988749894848204586834").parse().unwrap();
    let num_files = matches.value_of("NUM_FILES").unwrap_or("1").parse().unwrap();
    let divisor = matches.value_of("DIVISOR").unwrap_or("1").parse().unwrap();
    let silent = matches.is_present("SILENT");
    let header = matches.is_present("HEADER");
    let order = matches.occurrences_of("ORDER") as usize;
    let hl2_mode = matches.is_present("HL2");
    let ohlc_mode = matches.is_present("OHLC");
    let ohlcv_mode = matches.is_present("OHLCV");
    let f64_mode = matches.is_present("F64");
    let i64_mode = matches.is_present("I64");
    let u64_mode = matches.is_present("U64");

    enum Mode { HL2, OHLC, OHLCV, F64, I64, U64 }

    let go = |mode: Mode| {
        (match mode {
            Mode::HL2   => gen::<HL2>,
            Mode::OHLC  => gen::<OHLC>,
            Mode::OHLCV => gen::<OHLCV>,
            Mode::F64   => gen::<Vec<F>>,
            Mode::I64   => gen::<Vec<i64>>,
            Mode::U64   => gen::<Vec<u64>>,
        })(input, desired_len, output, chunking, chunk_delta, num_files, divisor, silent, header, order)
    };
    let ret = match (hl2_mode, ohlc_mode, ohlcv_mode, f64_mode, i64_mode, u64_mode) {
        (false, false, false, false, false, false) => go(Mode::F64),
        ( true, false, false, false, false, false) => go(Mode::HL2),
        (false,  true, false, false, false, false) => go(Mode::OHLC),
        (false, false,  true, false, false, false) => go(Mode::OHLCV),
        (false, false, false,  true, false, false) => go(Mode::F64),
        (false, false, false, false,  true, false) => go(Mode::I64),
        (false, false, false, false, false,  true) => go(Mode::U64),
        _ => Err(String::from("more than one modes selected!").into())
    };
    match ret {
        Ok(_) => if !silent { println!("done!") },
        Err(e) => println!("{}", e)
    }
}

fn gen<Row : Eq + Hash + Clone + Serialize + DeserializeOwned>
    ( input: &str
    , desired_len: usize
    , output: &str
    , chunking: usize
    , chunk_delta: f64
    , num_files: usize
    , divisor: usize
    , silent: bool
    , header: bool
    , order: usize
    ) -> Result<(), Box<dyn Error>> {

    if !silent { println!("reading history") }
    let start = Instant::now();
    let f = fs::read_to_string(input)?;
    let order = if order == 0usize { 1usize } else { order };
    let history_len = csv::ReaderBuilder::new().has_headers(header).from_reader(f.as_bytes()).deserialize::<Row>().count();
    let mut chunk_size = history_len / chunking;
    let mut chain = Chain::<Row>::of_order(order);
    let pb = ProgressBar::new({
        let mut acc = 0;
        let mut chunk_size = chunk_size;
        while chunk_size <= history_len / divisor {
            let mut skip_n = 0;
            while skip_n < history_len {
                acc += chunk_size;
                skip_n += chunk_size;
                chunk_size = (chunk_size as f64 * chunk_delta) as usize;
            }
        }
        acc as u64
    });
    let duration = start.elapsed();
    if !silent {
        println!("time elapsed reading history: {:?}", duration);
        println!("training MCMC:\n\t   len(history): {}\n\tmax(len(chunk)): {}", history_len, history_len / divisor)
    }
    let start = Instant::now();
    while chunk_size <= history_len / divisor {
        let mut skip_n = 0;
        while skip_n < history_len {
            let mut rdr = csv::ReaderBuilder::new().has_headers(header).from_reader(f.as_bytes());
            let results = rdr.deserialize::<Row>().skip(skip_n).take(chunk_size);
            let mut acc = vec![];
            for result in results {
                let row = result?;
                acc.push(row)
            }
            if acc.iter().count() > 0 { chain.feed(acc); }
            if !silent { pb.inc(chunk_size as u64) }
            skip_n += chunk_size;
            chunk_size = (chunk_size as f64 * chunk_delta) as usize;
        }
    }
    let duration = start.elapsed();
    if !silent { println!("time elasped training MCMC: {:?}", duration); }
    let pb = ProgressBar::new(num_files as u64);
    let gen = |i:Option<usize>| -> Result<(), Box<dyn Error>> {
        if !silent { println!("") }
        let mut wtr =
            csv::WriterBuilder::new()
                .has_headers(false)
                .from_path(match i {
                    Some(i) => {
                        if !silent { pb.inc(1); };
                        format!("{}.{}", i, output)
                    },
                    _ => { output.to_string() }})?;
        let mut count = 0usize;
        let mut last_elem = chain.generate().iter().next().unwrap().clone();
        let pb = ProgressBar::new(desired_len as u64);
        while count < desired_len {
            let data = chain.generate_from_token(last_elem);
            last_elem = data.iter().rev().next().unwrap().clone();
            let len = data.iter().count();
            count += len;
            for row in data.into_iter() { wtr.serialize(row)? }
            if !silent { match i { Some(_) => pb.inc(len as u64), _ => () } }
        }
        wtr.flush()?;
        Ok(())
    };
    let start = Instant::now();
    if !silent { println!("generating files") }
    for i in 1 .. num_files+1 { gen(if num_files > 1 { Some(i) } else { None })?;  }
    let duration = start.elapsed();
    if !silent { println!("time elapsed writing files: {:?}", duration); }
    Ok(())
}

#[derive(Debug, Clone, Copy)] struct F(ordered_float::OrderedFloat<f64>);

impl Hash for F { fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state) } }

impl PartialEq for F { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } }

impl Eq for F {}

impl Serialize for F { fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer { serializer.serialize_f64(*self.0) } }

impl<'de> Deserialize<'de> for F {
    fn deserialize<D>(deserializer: D) -> Result<F, D::Error> where D: Deserializer<'de> {
        let f = f64::deserialize(deserializer)?;
        Ok(F(ordered_float::OrderedFloat(f)))
    }
}
