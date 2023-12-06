use clap::Parser;
use fibonacci::*;

/// This program calculates the Fibonacci number at a specified index using the
/// fast doubling algorithm. The `fast_doubling_fibonacci` function leverages
/// properties of the Fibonacci sequence to efficiently compute large Fibonacci
/// numbers with reduced time complexity compared to naive recursive methods.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The index of the Fibonacci number to calculate.
    #[arg()]
    index: usize,
}

fn main() {
    let args = Args::parse();
    println!(
        "fibonacci({}) = {}",
        args.index,
        fast_doubling_fibonacci(args.index)
    );
}
