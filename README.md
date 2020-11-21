# Marky, the CSV file MCMC trainer

## [Preview of output generated from BTCUSD](https://sheganinans.github.io/marky/)

```sh
cargo build --release
./target/release/marky 500000 BTC.csv -c 1000
```

```
.\target\release\marky.exe --help
marky 0.0.2
Aistis Raulinaitis. <sheganians@gmail.com>
MCMC CSV Learner

USAGE:
    marky.exe [FLAGS] [OPTIONS] <DESIRED_LEN> <INPUT>

FLAGS:
        --floats     Raw float Mode (default true)
        --header     Has Header (default false)
        --hl2        HL2 Mode
        --ohlc       OHLC Mode
        --ohlcv      OHLCV Mode
    -d               Increase Order of MCMC
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chunking <CHUNKING>    Chunking Factor
    -t, --delta <CHUNK_DELTA>    Chunking Delta (default φ)
    -o, --output <OUTPUT>        Output Destination

ARGS:
    <DESIRED_LEN>    Desired Length of History
    <INPUT>          Input File
```
