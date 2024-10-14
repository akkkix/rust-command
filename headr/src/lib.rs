use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("author")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .default_value("-")
            .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
            .value_name("LINES")
            .short("n")
            .long("lines")
            .help("Lines")
            .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
            .value_name("BYTES")
            .short("c")
            .long("bytes")
            .help("Bytes")
            .takes_value(true)
            .conflicts_with("lines")
        )
        .get_matches();

    //let lines = match matches.value_of("lines") {
    //    Some(x) => parse_positive_int(x)?,
    //    _ => 10,
    //};

    let lines = parse_positive_int(matches.value_of("lines").unwrap())
        .map_err(|e| format!("illegal line count -- {}", e))?;

    //let bytes = match matches.value_of("bytes") {
    //    Some(x) => match parse_positive_int(x)
    //        {
    //            Ok(x) => Some(x),
    //            Err(x) => return Err(format!("illegal bytes error -- {}", x).into()),
    //        },
    //    _ => None,
    //};

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_num,filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    //let mut handle = file.take(num_bytes as u64);
                    //let mut buffer = vec![0; num_bytes];
                    //let bytes_read = handle.read(&mut buffer)?;
                    //print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    //let bytes: Result<Vec<_>, _> = file.bytes().take(num_bytes).collect();
                    let bytes = file.bytes().take(num_bytes).collect::<Result<Vec<_>, _>>();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse(){
        Ok(n) if n > 0 => Ok(n),
        //_ => Err(From::from()),
        //_ => Err(val.into()),
        _ => Err(Into::into(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

