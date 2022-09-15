extern crate bloom_filter; // not needed since Rust edition 2018

use bloom_filter::BloomFilter;
use clap::Parser;

use std::fmt::Debug;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Number of bits
    #[clap(short, long, value_parser)]
    m: usize,

    /// Number of elements
    #[clap(short, long, value_parser)]
    n: usize,
}

fn main() {
    let args = Arguments::parse();
    println!("Bloom filter example");
    let mut bfilter = BloomFilter::new_with_bits(args.m, args.n);

    let one = 1;
    let e0501 = "https://doc.rust-lang.org/error-index.html#E0501";
    let e0502 = "https://doc.rust-lang.org/error-index.html#E0502";
    bfilter.set(&e0501);
    bfilter.set(&one);
    println!("BloomFilter container 1: {}", bfilter.test(&one));
    println!("BloomFilter container {e0501}: {}", bfilter.test(&e0501));
    println!("BloomFilter container {e0502} : {}", bfilter.test(&e0502));
}
