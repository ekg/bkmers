use seq_io::fasta::Reader;
use debruijn::*;
use debruijn::kmer::*;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches= App::new("binary kmer emitter")
                          .version("0.1")
                          .author("Erik Garrison <erik.garrison@gmail.com>")
                          .about("Emits 8, 16, 32 and 64-bit kmers using a 2-bit DNA alphabet")
                          .arg(Arg::with_name("kmer")
                               .short("k")
                               .long("kmer-length")
                               .value_name("K")
                               .help("Sets the kmer length, must be 4, 8, 16, or 32.")
                               .takes_value(true))
                          .arg(Arg::with_name("INPUT")
                               .help("FASTA file to use")
                               .required(true)
                               .index(1))
                          .get_matches();

    let kmer = matches.value_of("kmer").unwrap_or("16").parse::<u32>().unwrap();

    let input = matches.value_of("INPUT").unwrap();

    make_kmers(input, kmer)
}

fn make_kmers(input: &str, kmer: u32) {

    let mut reader = Reader::from_path(input).unwrap();

    while let Some(result) = reader.next() {
        let record = result.unwrap();
        let seq = record.full_seq();
        // Generate a set of kmers from a string, then sort
        match kmer {
            4 => for k in Kmer4::kmers_from_ascii(&seq) {
                println!("{}", k.storage);
            },
            8 => for k in Kmer8::kmers_from_ascii(&seq) {
                println!("{}", k.storage);
            },
            16 => for k in Kmer16::kmers_from_ascii(&seq) {
                println!("{}", k.storage);
            },
            32 => for k in Kmer32::kmers_from_ascii(&seq) {
                println!("{}", k.storage);
            },
            _ => panic!("invalid kmer size {}", kmer)
        }
    }
}
