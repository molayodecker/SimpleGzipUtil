extern crate flate2;

// use flate2::bufread::GzEncoder;
use flate2::write::GzEncoder as GzWriter;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;
use std::env::args;
use std::io::copy;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Instant;

fn encorder(input_file: &str, output_file: &str) {
    let start = Instant::now();
    let input = File::open(input_file).unwrap();
    let mut input = BufReader::new(input);
    let output = File::create(output_file).unwrap();
    let mut encoder = GzWriter::new(output, Compression::default());
    copy(&mut input, &mut encoder).unwrap();
    println!("Compression complete!");
    println!("Output file: {}", "output.gz");
    let duration = start.elapsed();
    println!("Compressed in {:?}", duration);
    // Flush the encoder to ensure all data is written
    encoder.finish().unwrap();
}

fn decoder(input_file: &str, output_file: &str) {
    let start = Instant::now();
    let input = File::open(input_file).unwrap();
    let output = File::create(output_file).unwrap();
    let mut output = BufWriter::new(output);
    let mut decoder = GzDecoder::new(input);
    copy(&mut decoder, &mut output).unwrap();
    println!("Decompression complete!");
    println!("Output file: {}", output_file);
    let duration = start.elapsed();
    println!("Decompressed in {:?}", duration);
}


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.gz> <output>", args[0]);
        std::process::exit(1);
    }
    println!("Arguments: {:?}", args.iter().collect::<Vec<_>>());
    let input_file = &args[1];
    let output_file = &args[2];

    encorder(input_file, output_file);


}