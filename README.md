# king_david

This is a fuzzing library, like LibAFL.
It aims to be easier to understand and use than LibAFL, maybe at the expense of performance.

## getting started

```sh
./build.bash && cargo run --release --example coverage_guided_stdin_fuzzer ./example_targets/strcmp_afl/main
```
```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
~/code/king_david/example_targets/crash_when_stdin_is_ff ~/code/king_david
make: Nothing to be done for 'all'.
~/code/king_david
~/code/king_david/example_targets/strcmp_afl ~/code/king_david
make: Nothing to be done for 'all'.
~/code/king_david
   Compiling king_david v0.1.0 (/home/bkallus/code/king_david)
    Finished `release` profile [optimized] target(s) in 0.54s
     Running `target/release/examples/coverage_guided_stdin_fuzzer ./example_targets/strcmp_afl/main`
Found result: [100, 97, 118, 101]
```

## name

Named after my cat, [David](https://kallus.org/img/dave.jpeg).
