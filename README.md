# rust-fibonacci
How to calculate the fibonacci sequence in Rust with a Recursion, memoized solution and bottom up approached.

![Benchmark Test](benchmark.png)


Note: This example runs on Rust nightly because it uses the unstable test feature for benchmarking. 

## Solutions
This repository comes with three different solutions of how to solve the fibonacci sequence:
- [Recursive Fibonacci O(2^N)](src/lib.rsL#13)
- [Recursive Fibonacci with Memoization O(N) ( Dynamic Programming Solution )](src/lib.rs#L32)
- [Bottom up approach O(N)](src/lib.rs#L59)

## Install Rust Nightly
```bash
rustup install nightly
rustup default nightly
```

## Run the tests
```bash
cargo test
```

## Run the benchmark tests
```bash
cargo bench
```
