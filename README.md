# Marky, the CSV file MCMC trainer

## [Preview of output generated from BTCUSD](https://sheganinans.github.io/marky/)

```sh
cargo build --release
./target/release/marky 500000 BTC.csv -c 1000
```

```
 .\target\release\marky.exe --help
marky 0.0.4
Aistis Raulinaitis. <sheganians@gmail.com>
MCMC CSV Learner

USAGE:
    marky.exe [FLAGS] [OPTIONS] <DESIRED_LEN> <INPUT>

FLAGS:
        --f64        raw f64 mode (default true)
        --header     has header (default false)
        --hl2        HL2 mode
        --i64        i64 mode
        --ohlc       OHLC mode
        --ohlcv      OHLCV mode
    -d               increase order of MCMC
        --u64        u64 mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chunking <CHUNKING>    chunking factor (default 10)
    -t, --delta <CHUNK_DELTA>    chunking delta (default φ)
    -n, --num <NUM_FILES>        generate n mumber of files named n.out.csv
    -o, --output <OUTPUT>        output destination

ARGS:
    <DESIRED_LEN>    desired length of history
    <INPUT>          input file
```
