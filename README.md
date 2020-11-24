# marky, the CSV time series MCMC trainer

Generate random walks easier than ever!

## [500,000 minutes of generated BTCUSD price action](https://sheganinans.github.io/marky/)

```
$ cargo build --release
$ ./target/release/marky 500000 BTC.csv
reading history
training MCMC
██████████████████████████████████████████████████████████████████████████████████ 1343542/1343664
time elasped training MCMC: 109.139263s
generating files
done!
```

```
./target/release/marky -h
marky 0.0.7
Aistis Raulinaitis. <sheganians@gmail.com>
marky, the CSV time series MCMC trainer

USAGE:
    marky.exe [FLAGS] [OPTIONS] <DESIRED_LEN> <INPUT>

FLAGS:
        --f64        f64 mode (default true)
        --header     has header (default false)
        --hl2        HL2 mode
        --i64        i64 mode
        --ohlc       OHLC mode
        --ohlcv      OHLCV mode
    -d               increase order of MCMC
    -s, --silent     make me shut up
        --u64        u64 mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --num <NUM_FILES>    generate n mumber of files named `n.out.csv`
    -o, --output <OUTPUT>    output destination

ARGS:
    <DESIRED_LEN>    desired length of output file
    <INPUT>          input file
```

## Details

```
      HL2 mode expects exactly 2 columns :                f64, u64
     OHLC mode expects exactly 4 columns : f64, f64, f64, f64
    OHLCV mode expects exactly 5 columns : f64, f64, f64, f64, u64
```

Otherwise `--f64`, `--i64`, and `--u64` expect a homogeneous matrix of that type of any row length
