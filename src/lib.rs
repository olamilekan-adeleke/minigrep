use std::error::Error;
use std::fs;

pub mod config;
pub use config::Config;
use config::ConfigOptions;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name)?;

    let lines = match config.option {
        ConfigOptions::CaseInSensitive => search(&config.query, &file_content),
        ConfigOptions::CaseSensitive => vec![],
        ConfigOptions::CountOccurrence => vec![],
        ConfigOptions::WithLineNumber => vec![],
        ConfigOptions::None => vec![],
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
                        Rust:
                        safe, fast, productive.
                        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
