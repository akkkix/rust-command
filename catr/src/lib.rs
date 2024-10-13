use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                //println!("Opened {}", filename);
                let mut blank = 0;
                for (i,line) in file.lines().enumerate() {
                    let line = line?;
                    if config.number_lines {
                        println!("{:>6}\t{}",i+1,line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("");
                            blank+=1;
                        } else {
                            println!("{:>6}\t{}",i+1-blank,line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    //println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("author-x")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .default_value("-")
            .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
            .short("n")
            .long("number")
            .help("Number lines")
            .conflicts_with("number_nonblank_lines")
            .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
            .short("b")
            .long("number-nonblank")
            .help("Number nonblank lines")
            .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
