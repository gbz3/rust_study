use std::error::Error;
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short = 'a', long = "ascii",
        help = "ASCII")]
    ascii: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    //println!("{:#?}", config);

    let char_ranges = match config {
        Config {ascii: true, ..} => vec!['\x00'..='\x7F'],
        _ => vec!['\u{0}'..='\u{D7FF}', '\u{E000}'..='\u{10FFFF}'],
    };

    for chars in &char_ranges {
        chars.clone().into_iter()
            .filter(|ch| !ch.is_control())
            .for_each(|ch| println!("{}", ch));
    }

    Ok(())
}
