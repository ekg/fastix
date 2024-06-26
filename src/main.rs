#![allow(clippy::too_many_arguments)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

extern crate clap;
use clap::{Command, Arg};

fn for_each_line_in_fasta(fasta_filename: &str, mut callback: impl FnMut(String)) {
    if fasta_filename == "-" {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            callback(line.unwrap());
        }
    }
    else {
        let file = File::open(fasta_filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            callback(line.unwrap());
        }
    }
}

fn process_fasta(fasta_filename: &str, prefix: &str, uppercase: bool) {
    for_each_line_in_fasta(fasta_filename, |mut l: String| {
        if !l.is_empty() {
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
    let matches = Command::new("fastix")
        .version("0.1.0")
        .author("Erik Garrison <erik.garrison@gmail.com>")
        .about("Trivial manipulations of FASTA files")
        .arg(
            Arg::new("INPUT")
                .required(true)
                .num_args(1)
                .index(1)
                .help("input FASTA file"),
        )
        .arg(
            Arg::new("prefix")
                .short('p')
                .long("prefix")
                .num_args(1)
                .help("Prefix to add to each record in the file"),
        )
        .get_matches();

    let filename = matches.get_one::<String>("INPUT").unwrap();

    let prefix = matches.get_one::<String>("prefix").unwrap();

    process_fasta(filename, prefix, true);

    Ok(())
}
