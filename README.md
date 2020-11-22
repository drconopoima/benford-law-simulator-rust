# Benford's law leading digit simulator in typescript

A simulator for leading digit data collections following Benford's Law. Built in Rust.

## Installation & build

```sh
  git clone https://github.com/drconopoima/benford-law-simulator-rust.git
  cd benford-law-simulator-rust
  cargo build --release
```

## Run

You can assign a top of the range `--max` (`-m`), and a bottom of range `--min` (`-n`) for the simulations manually:

```sh
car run --release -- --max 12345 --lead-digits 1 \
  --samplecount 1000 --min 1
```

You can also generate a random number in a range with shuf, from coreutils library, or by any other random number generator.

```sh
cargo run --release -- --lead-digits 1 --sample-size 10000 \
  --min 1 --max $(shuf -i 10-100000 -n 1|tee >&2|cat)
```

The expected output would be like the following:

```txt
50547
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/benford-law-simulator-rust --lead-digits 1
     --sample-size 10000 --min 1 --max 50547`
Lead digits counts:
[(1.0, 2214.0), (2.0, 1848.0), (3.0, 1632.0), (4.0, 1490.0), (5.0, 1313.0),
(6.0, 797.0), (7.0, 249.0), (8.0, 226.0), (9.0, 231.0)]
⡇⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⡁ 2214.0
⠧⠤⠤⠤⠤⠤⠤⢤                                                   ⠄
⠂       ⠒⠒⠒⠒⠒⠒⠒⡆                                           ⠂
⡁               ⠉⠉⠉⠉⠉⢹⣀⣀⣀⣀⣀⣀⣀                             ⡁
⠄                             ⡇                             ⠄
⠂                             ⣇⣀⣀⣀⣀⣀⣀                      ⠂
⡁                                    ⢸                      ⡁
⠄                                    ⢸                      ⠄
⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉ 226.0
1.0                                                      9.0
```

The upper boundary for the generated data was set at 50547 in the output seen above.

You may also run the binary release build directly, by using its path

```sh
$ ./target/release/benford-law-simulator-rust -s 10 --min 1 \
  -m 307212 --lead-digits 1
Lead digits counts:
[(1.0, 3.0), (2.0, 2.0), (3.0, 1.0), (4.0, 1.0), (7.0, 1.0),
(8.0, 2.0)]
⡇⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⠁⠈ ⡁ 3.0
⡇                                                           ⠄
⡇                                                           ⠂
⡇                                                           ⡁
⠍⠉⠉⠉⠉⠉⢹                                     ⡏⠉⠉⠉⠉⠉⠉⠉      ⠄
⠂      ⢸                                     ⡇              ⠂
⡁      ⢸                                     ⡇              ⡁
⠄      ⢸                                     ⡇              ⠄
⠁⠈ ⠁⠈ ⠁⠈⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁⠈ ⠁⠈ ⠁⠈ 1.0
1.0                                                      9.0
```

If you want to get the generated sample, you may pass optional `--debug (-d)` flag:

```sh
$ benford-law-simulator-rust --sample-size 10 -n 1 \
  --max 307212 -l 1
Simulated sample data:
[86461, 37359, 202275, 81769, 71067, 43553, 294327, 128148,
136278, 17016]
Lead digits:
[8, 3, 2, 8, 7, 4, 2, 1, 1, 1]
...
```

You may generate output for the lead pair of digits as well, or any lead digit count with the option `--lead-digits (-l)`

```sh
$ cargo run --release -- --lead-digits 2 -m 10000 -n 10 -s 1000
```
