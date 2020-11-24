#[macro_use] extern crate clap;

use std::{error::Error};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use indicatif::ProgressBar;
use linecount::count_lines;
use markov::Chain;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de::DeserializeOwned};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct   HL2 {                   p: F, v: u64 }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct  OHLC { o: F, h: F, l: F, c: F }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct OHLCV { o: F, h: F, l: F, c: F, v: u64 }

fn main() {
    let matches = clap_app!(marky =>
        (version: "0.0.8")
        (author: "Aistis Raulinaitis. <sheganians@gmail.com>")
        (about: "marky, the CSV time series MCMC trainer")
        (@arg DESIRED_LEN: +required "desired length of output file")
        (@arg INPUT: +required "input file")
        (@arg OUTPUT: -o --output +takes_value "output destination")
        (@arg ORDER: -r --order +takes_value "increase order of MCMC")
        (@arg NUM_FILES: -n --num +takes_value "generate n mumber of files named `n.out.csv`")
        (@arg CHUNK_SIZE: -c --chunk +takes_value "initial chunk size (default 100)")
        (@arg CHUNK_DELTA: -d --delta +takes_value "chunking delta (default φ)")
        (@arg DIVISOR: -v --divisor +takes_value "`max(len(chunk)) < len(history)/divisor` (default 1)")
        (@arg SILENT: -s --silent "make me shut up")
        (@arg HEADER: --header "has header (default false)")
        (@arg HL2: --hl2 "HL2 mode")
        (@arg OHLC: --ohlc "OHLC mode")
        (@arg OHLCV: --ohlcv "OHLCV mode")
        (@arg F64: --f64 "f64 mode (default true)")
        (@arg I64: --i64 "i64 mode")
        (@arg U64: --u64 "u64 mode")
    ).get_matches();

    let desired_len = matches.value_of("DESIRED_LEN").unwrap().parse().unwrap();
    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("out.csv");
    let order = matches.value_of("ORDER").unwrap_or("1").parse().unwrap();
    let num_files = matches.value_of("NUM_FILES").unwrap_or("1").parse().unwrap();
    let init_chunk_size : usize = matches.value_of("CHUNK_SIZE").unwrap_or("100").parse().unwrap();
    let chunk_delta : f64 = matches.value_of("CHUNK_DELTA").unwrap_or("1.618033988749894848204586834").parse().unwrap();
    let divisor : usize = matches.value_of("DIVISOR").unwrap_or("1").parse().unwrap();
    let silent = matches.is_present("SILENT");
    let header = matches.is_present("HEADER");
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
        })(desired_len, input, output, num_files, init_chunk_size, chunk_delta, divisor, silent, header, order)
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
    ( desired_len: usize
    , input: &str
    , output: &str
    , num_files: usize
    , init_chunk_size: usize
    , chunk_delta: f64
    , divisor: usize
    , silent: bool
    , header: bool
    , order: usize
    ) -> Result<(), Box<dyn Error>> {

    let order = if order == 0usize { 1usize } else { order };
    let mut chain = Chain::<Row>::of_order(order);
    let start = Instant::now();
    let history_len = { let file = File::open(input)?; count_lines(file)? };
    let training_len = {
        let mut acc = 0;
        let mut chunk_size = init_chunk_size;
        while chunk_size < history_len / divisor {
            acc += 1;
            chunk_size = (chunk_size as f64 * chunk_delta) as usize;
        }
        acc
    };
    let pb = ProgressBar::new(training_len as u64);
    if !silent {
        println!("training MCMC\n\t   len(history): {}", history_len);
        println!("\tmax(len(chunk)): {}", history_len / divisor);
        println!("\tmin(len(chunk)): {}", init_chunk_size);
    }
    let mut chunk_size = init_chunk_size;
    while chunk_size < history_len / divisor {
        let file = File::open(input)?;
        let mut rdr = csv::ReaderBuilder::new().has_headers(header).from_reader(file);
        loop {
            let mut acc = vec![];
            for result in rdr.deserialize::<Row>().take(chunk_size) {
                let row = result?;
                acc.push(row)
            }
            if acc.iter().count() == 0 { break; }
            chain.feed(&acc);
        }
        pb.inc(1);
        chunk_size = (chunk_size as f64 * chunk_delta) as usize;
    }

    let duration = start.elapsed();
    if !silent { println!("time elasped training MCMC: {:?}", duration); }
    let gen = |i: Option<usize>| -> Result<(), Box<dyn Error>> {
        let mut wtr =
            csv::WriterBuilder::new()
                .has_headers(false)
                .from_path(match i {
                    Some(i) => format!("{}.{}", i, output),
                    _ => { output.to_string() }})?;
        let mut last_elem = chain.generate().into_iter().last().unwrap();
        let mut count = 0;
        while count < desired_len {
            let data = chain.generate_from_token(last_elem.clone());
            for row in data.iter() { wtr.serialize(row)? }
            count = count + data.iter().count();
            last_elem = data.into_iter().last().unwrap();
        }
        wtr.flush()?;
        Ok(())
    };
    let start = Instant::now();
    if !silent { println!("generating files") }
    let pb = ProgressBar::new(num_files as u64);
    for i in 1 .. num_files + 1 {
        let multiple = num_files > 1;
        gen(if multiple { Some(i) } else { None })?;
        if !silent { if multiple { pb.inc(1) } }
    }
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
