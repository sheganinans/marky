# Marky, the CSV file MCMC trainer

## [Preview of output generated from BTCUSD](https://sheganinans.github.io/marky/)

```sh
rustup run nightly cargo build --release
./target/release/marky 500000 .\BTC.csv -c 1000
```

```
.\target\release\marky.exe --help
marky 0.0.1
Aistis Raulinaitis. <sheganinans@gmail.com>
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
    -o, --output <OUTPUT>        Output Destination

ARGS:
    <DESIRED_LEN>    Desired Length of History
    <INPUT>          Input File
```
