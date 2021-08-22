use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_configs(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents: String = fs::read_to_string(config.filename).expect("Unable to read from file");

    println!("Contents are as follows:\n\n{}", contents);
}

struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

fn parse_configs(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];

    Config { query, filename }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
