use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }

        let query = &args[1];
        let filename = &args[2];
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;

    println!("Contents are as follows:\n\n{}", contents);

    Ok(())
}
