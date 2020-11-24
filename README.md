# marky, the CSV time series MCMC trainer

Generate random walks easier than ever!

## [500,000 minutes of generated BTCUSD price action](https://sheganinans.github.io/marky/)

```
$ cargo build --release
$ ./target/release/marky 500000 BTC.csv -n 3 -v 10
training MCMC
           len(history): 1343664
        max(len(chunk)): 134366
        min(len(chunk)): 100
██████████████████████████████████████████████████████████████████████████████████ 15/15
time elasped training MCMC: 41.6897262s
generating files
done!
```

```
./target/release/marky -h
marky 0.0.9
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
    -s, --silent     make me shut up
        --u64        u64 mode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delta <CHUNK_DELTA>    chunking delta (default φ)
    -c, --chunk <CHUNK_SIZE>     initial chunk size (default 100)
    -v, --divisor <DIVISOR>      `max(len(chunk)) < len(history)/divisor` (default 1)
    -i, --init <INIT_VAL>        give an initial value to each file's MCMC
    -n, --num <NUM_FILES>        generate n mumber of files named `n.out.csv`
    -r, --order <ORDER>          increase order of MCMC
    -o, --output <OUTPUT>        output destination

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
