use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name)?;
    println!("With text:\n{:?}", file_content);

    Ok(())
}
