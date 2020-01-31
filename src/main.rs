#![allow(clippy::too_many_arguments)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

extern crate clap;
use clap::{App, Arg};

fn for_each_line_in_fasta(fasta_filename: &str, mut callback: impl FnMut(String)) {
    let file = File::open(fasta_filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        callback(line.unwrap());
    }
}

fn process_fasta(fasta_filename: &str, prefix: &str, uppercase: bool) {
    for_each_line_in_fasta(fasta_filename, |mut l: String| {
        if l.len() > 0 {
            if l.chars().nth(0).unwrap() == '>' {
                let basename = l.split('>').nth(1).unwrap().split(' ').nth(0).unwrap();
                println!(">{}{}", prefix, basename);
            } else if uppercase {
                l.make_ascii_uppercase();
                println!("{}", l);
            } else {
                println!("{}", l);
            }
        }
    });
}

fn main() -> io::Result<()> {
    let matches = App::new("fastix")
        .version("0.1.0")
        .author("Erik Garrison <erik.garrison@gmail.com>")
        .about("Trivial manipulations of FASTA files")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("input FASTA file"),
        )
        .arg(
            Arg::with_name("prefix")
                .short("p")
                .long("prefix")
                .takes_value(true)
                .help("Prefix to add to each record in the file"),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();

    let prefix = matches.value_of("prefix").unwrap();

    process_fasta(filename, prefix, true);

    Ok(())
}
