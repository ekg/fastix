# fastix

A simple command line tool to add prefixes to FASTA headers.
The idea is to support pangenomic applications, following the [PanSN](https://github.com/pangenome/PanSN-spec) hierarchical naming specification.

## Features

- Add a prefix to FASTA record names
- Convert sequences to uppercase

## Usage

```
fastix [OPTIONS] <INPUT>

Arguments:
  <INPUT>             Input FASTA file

Options:
  -h, --help          Print help information
  -V, --version       Print version information
  -p, --prefix <STR>  Prefix to add to FASTA record names
```

## Example

```
$ fastix -p "gen#1#" genome.fa > genome_prefixed.fa
```

This will add "gen#1#" as a prefix to all FASTA record names in genome.fa.

## Installation

```
cargo install fastix
```

## License

This project is licensed under the MIT license. See LICENSE for more details.
