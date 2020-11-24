# marky, the CSV time series MCMC trainer

Generate random walks easier than ever!

## [500,000 minutes of generated BTCUSD price action](https://sheganinans.github.io/marky/)

```
cargo build --release
./target/release/marky 500000 BTC.csv -c 1000
reading history
time elapsed reading history: 66.8794ms
training MCMC:
           len(history): 1343664
        max(len(chunk)): 1343664
        min(len(chunk)): 1343
██████████████████████████████████████████████████████████████████████████████████ 4791082/4791082
time elasped training MCMC: 7.5264929s
generating files

done!
```

```
./target/release/marky -h
marky 0.0.6
Aistis Raulinaitis. <sheganians@gmail.com>
marky, the CSV time series MCMC trainer

USAGE:
    marky.exe [FLAGS] [OPTIONS] <DESIRED_LEN> <INPUT>

FLAGS:
        --f64        f64 mode (default true)
        --header     has header (default false)
        --hl2        HL2 mode
        --i64        i64 mode
        --ohlcv      OHLCV mode
    -d               increase order of MCMC
    -s, --silent     make me shut up
        --u64        u64 mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chunking <CHUNKING>    chunking factor (default 10)
    -t, --delta <CHUNK_DELTA>    chunking delta (default φ)
    -r, --divisor <DIVISOR>      set `max(len(chunks)) < len(rows)/divisor` (useful for large files)
    -n, --num <NUM_FILES>        generate n mumber of files named `n.out.csv`
    -o, --output <OUTPUT>        output destination

ARGS:
    <DESIRED_LEN>    desired length of history
    <INPUT>          input file
```

## Details

```
      HL2 mode expects exactly 2 columns :                f64, u64
     OHLC mode expects exactly 4 columns : f64, f64, f64, f64
    OHLCV mode expects exactly 5 columns : f64, f64, f64, f64, u64
```

Otherwise `--f64`, `--i64`, and `--u64` expect a homogeneous matrix of that type of any row length
