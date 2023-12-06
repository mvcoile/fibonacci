# Fibonacci Project

## Overview
This project implements a fast algorithm to calculate Fibonacci numbers at a specified index. It uses the fast doubling algorithm, which is more efficient than the naive recursive approach, especially for large indices.

## Usage
To use this program, you need to pass the index of the Fibonacci number you want to calculate as a command-line argument.

Example:
```
cargo run -- 10
```
This will calculate the Fibonacci number at index 10.

## Building
To build the project, use the following command:
```
cargo build --release
```

## Testing
To run tests, use the following command:
```
cargo test
```

## Benchmarking
To run benchmarking, use the following command:
```
cargo bench
```
