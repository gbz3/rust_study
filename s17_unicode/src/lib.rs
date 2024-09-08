use std::error::Error;
use std::fmt;
use std::fmt::{Formatter};
use std::ops::RangeInclusive;
use std::str::FromStr;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
enum MyError {
    InvalidCharRange,
}

impl Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InvalidCharRange => write!(f, "invalid character range"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short = 'a', long = "ascii",
        help = "ASCII")]
    ascii: bool,

    #[arg(short = 'u', long = "unicode",
        help = "print specified unicode range")]
    unicode_range: Option<String>,

    #[arg(short = 'c', long = "char",
        help = "print specified character range")]
    char_range: Option<String>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);

    let char_ranges = match config {
        Config { unicode_range: Some(code), .. } => match parse_unicode_range(&code, '-') {
            Some(v) => v,
            _ => return Err(Box::new(MyError::InvalidCharRange))
        },
        Config {char_range: Some(code), ..} => match parse_pair::<String>(&code, '-') {
            Some((l, r)) => vec![char::from_str(&l)?..=char::from_str(&r)?],
            _ => return Err(Box::new(MyError::InvalidCharRange))
        },
        Config {ascii: true, ..} => vec!['\x00'..='\x7F'],
        _ => vec!['\u{0}'..='\u{D7FF}', '\u{E000}'..='\u{10FFFF}'],
    };

    for chars in &char_ranges {
        chars.clone().into_iter()
            .filter(|ch| !ch.is_control())
            .for_each(|ch| println!("{:#08x}: [{}]", ch as u64, ch));
    }

    Ok(())
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        },
    }
}

fn parse_unicode_range(s: &str, separator: char) -> Option<Vec<RangeInclusive<char>>> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (
                u32::from_str_radix(&s[..index], 16).ok().and_then(|n| char::from_u32(n)),
                u32::from_str_radix(&s[index + 1..], 16).ok().and_then(|n| char::from_u32(n))
            ) {
                (Some(l), Some(r)) => Some(vec![l..=r]),
                _ => None,
            }
        }
    }
}
