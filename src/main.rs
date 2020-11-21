#[macro_use] extern crate clap;

use std::error::Error;
use std::fs;
use std::hash::{Hash, Hasher};

use markov::Chain;
use serde::{Deserialize, Serialize, Serializer, Deserializer, de::DeserializeOwned};
use rayon::prelude::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct HL2 { p: F, v: u64 }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct OHLC { o: F, h: F, l: F, c: F }
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)] struct OHLCV { o: F, h: F, l: F, c: F, v: u64 }

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

fn gen<T : Eq + Hash + Clone + Sync + Serialize + DeserializeOwned>
    (input: &str, desired_len: usize, output: &str, chunking: usize, chunk_delta: f64, num_files: u64, header: bool, order: usize) -> Result<(), Box<dyn Error>> {
    println!("Reading History");
    let mut acc = vec![];
    let f = fs::read_to_string(input)?;
    let order = if order == 0usize { 1usize } else { order };
    let mut rdr = csv::ReaderBuilder::new().has_headers(header).from_reader(f.as_bytes());
    for result in rdr.deserialize() { let row: T = result?; acc.push(row) }
    println!("Training MCMC");
    let mut chain = Chain::of_order(order);
    let history_len = acc.iter().count();
    let mut chunk_size = history_len / chunking;
    while chunk_size < history_len {
        chunk_size = (chunk_size as f64 * chunk_delta) as usize;
        for d in acc[..].chunks(chunk_size as usize) { chain.feed(d); }
    }
    println!("Generating Files");
    let gen = |i:Option<u64>| -> Result<(), Box<dyn Error>> {
        let mut wtr =
            csv::WriterBuilder::new()
                .has_headers(false)
                .from_path(match i { Some(i) => format!("{}.{}", i, output), _ => output.to_string() })?;
        let mut count = 0usize;
        let mut last_elem = chain.generate().iter().next().unwrap().clone();
        while count < desired_len {
            let data = chain.generate_from_token(last_elem);
            last_elem = data.iter().rev().next().unwrap().clone();
            count += data.iter().count();
            for row in data.into_iter() { wtr.serialize(row)? }
        }
        wtr.flush()?;
        Ok(())
    };
    (1..num_files+1).into_par_iter().for_each(|i| { gen(if num_files > 1 {Some(i)} else {None}).unwrap() });
    Ok(())
}

fn main() {
    let matches = clap_app!(marky =>
        (version: "0.0.3")
        (author: "Aistis Raulinaitis. <sheganians@gmail.com>")
        (about: "MCMC CSV Learner")
        (@arg DESIRED_LEN: +required "Desired Length of History")
        (@arg INPUT: +required "Input File")
        (@arg OUTPUT: -o --output +takes_value "Output Destination")
        (@arg CHUNKING: -c --chunking +takes_value "Chunking Factor (default 10)")
        (@arg CHUNK_DELTA: -t --delta +takes_value "Chunking Delta (default φ)")
        (@arg NUM_FILES: -n --num +takes_value "Generate n mumber of files named n.out.csv")
        (@arg HEADER: --header "Has Header (default false)")
        (@arg ORDER: -d ... "Increase Order of MCMC")
        (@arg HL2: --hl2 "HL2 Mode")
        (@arg OHLC: --ohlc "OHLC Mode")
        (@arg OHLCV: --ohlcv "OHLCV Mode")
        (@arg FLOATS: --floats "Raw float Mode (default true)")
    ).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let desired_len : usize = matches.value_of("DESIRED_LEN").unwrap().parse().unwrap();
    let output = matches.value_of("OUTPUT").unwrap_or("out.csv");
    let chunking : usize = matches.value_of("CHUNKING").unwrap_or("10").parse().unwrap();
    let chunk_delta : f64 = matches.value_of("CHUNK_DELTA").unwrap_or("1.618033988749894848204586834").parse().unwrap();
    let num_files : u64 = matches.value_of("NUM_FILES").unwrap_or("1").parse().unwrap();
    let header = matches.is_present("HEADER");
    let order = matches.occurrences_of("ORDER") as usize;
    let hl2_mode = matches.is_present("HL2");
    let ohlc_mode = matches.is_present("OHLC");
    let ohlcv_mode = matches.is_present("OHLCV");
    let floats_mode = matches.is_present("FLOATS");

    enum Mode { HL2, OHLC, OHLCV, Floats }

    let go = |mode: Mode| {
        (match mode {
            Mode::HL2 => gen::<HL2>,
            Mode::OHLC => gen::<OHLC>,
            Mode::OHLCV => gen::<OHLCV>,
            Mode::Floats => gen::<Vec<F>>,
        })(input, desired_len, output, chunking, chunk_delta, num_files, header, order)
    };
    let ret = match (hl2_mode, ohlc_mode, ohlcv_mode, floats_mode) {
        (false, false, false, false) => go(Mode::Floats),
        (true, false, false, false) => go(Mode::HL2),
        (false, true, false, false) => go(Mode::OHLC),
        (false, false, true, false) => go(Mode::OHLCV),
        (false, false, false, true) => go(Mode::Floats),
        _ => Ok(())
    };
    match ret {
        Ok(_) => println!("Done!"),
        Err(e) => println!("{}",e)
    }
}
