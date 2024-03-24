
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
    number_of_lines: u32,

    #[arg(short, long, default_value_t = 0)]
    character_count: u32,
}

fn main() {
    let args = Args::parse();

    if args.fnames.len() > 1 {
        for fname in args.fnames {
            println!("==> {} <==", fname);
            output_head_of_file(args.character_count, args.number_of_lines, &fname);
            println!();
        }
    } else {
        output_head_of_file(args.character_count, args.number_of_lines, &args.fnames[0]);
    }
}

fn output_head_of_file(character_count: u32, number_of_lines: u32, fname: &str) {
    let mut reader = BufReader::new(File::open(fname).expect("Opening fname failed"));
    if character_count > 0 {
        let mut iterator = 0;
        for line in reader.lines() {
            for character in line.expect("Reading lines failed").chars() {
                if iterator >= character_count {
                    break;
                }
                print!("{}", character);
                iterator+=1;
            }
            println!(); // adds newline back in - feels hacky, will try and fix

            if iterator >= character_count {
                break;
            }
        }
    } else {
        for _ in 0..number_of_lines {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            println!("{}", line);
        }
    }
}
