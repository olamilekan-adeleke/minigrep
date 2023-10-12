use std::error::Error;
use std::fs;

pub enum ConfigOptions {
    CaseSensitive,
    CaseInSensitive,
    CountOccurrence,
    WithLineNumber,
    None,
}

pub struct Config {
    query: String,
    file_name: String,
    option: ConfigOptions,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minigrep [options] [query] [filename]");
        }

        let option_or_query = args[1].clone();

        let query;
        let file_name;
        let config_option;

        if option_or_query.starts_with("-") {
            config_option = Config::prase_str_to_options(args[1].clone());
            query = args[2].clone();
            file_name = args[3].clone();
        } else {
            config_option = ConfigOptions::None;
            query = args[1].clone();
            file_name = args[2].clone();
        }

        Ok(Config {
            query,
            file_name,
            option: config_option,
        })
    }

    fn prase_str_to_options(option: String) -> ConfigOptions {
        match option.as_str() {
            "-i" => ConfigOptions::CaseSensitive,
            "-c" => ConfigOptions::CountOccurrence,
            "-n" => ConfigOptions::WithLineNumber,
            _ => ConfigOptions::CaseInSensitive,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name)?;

    for line in search(&config.query, &file_content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }

    results
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
