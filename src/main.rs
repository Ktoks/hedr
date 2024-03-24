
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hedr")]
#[command(about = "Print the first 10 lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n\nWith no FILE, or when FILE is -, read standard input.")]
#[command(long_about = None)]
#[command(version = ".1")]
struct Args {
    fnames: Vec<String>,

    #[arg(short, long, default_value_t = 10)]
    num_of_lines: u8,

    #[arg(short, long, default_value_t = 0)]
    character_length: u8,
}

fn main() {
    let args = Args::parse();

    if args.fnames.len() > 1 {
        for fname in args.fnames {
            println!("==> {} <==", fname);
            some(args.character_length, args.num_of_lines, &fname);
            println!();
        }
    } else {
        some(args.character_length, args.num_of_lines, &args.fnames[0]);
    }
}

fn some(character_length: u8, num_of_lines: u8, fname: &str) {
    let mut reader = BufReader::new(File::open(fname).expect("open failed"));
    if character_length > 0 {
        let mut iterator = 0;
        for line in reader.lines() {
            for car in line.expect("lines failed").chars() {
                if iterator >= character_length {
                    break;
                }
                print!("{}", car);
                iterator+=1;
            }
            if iterator >= character_length {
                break;
            }
        }
    } else {
        for _ in 0..num_of_lines {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            println!("{}", line);
        }
    }
}
