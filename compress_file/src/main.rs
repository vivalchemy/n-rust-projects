extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        println!("Usage: <source> <destination>");
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();

    let output = encoder.finish().unwrap();

    println!(
        "Source length: {}",
        input.get_ref().metadata().unwrap().len()
    );

    println!("Target length: {}", output.metadata().unwrap().len());

    print!("Time taken: {:?}", start.elapsed());
}
