use std::error::Error;
use std::fs;

mod config;
pub use config::Config;
use config::ConfigOptions;

mod action_type;
use action_type::search_case_insensitive;
use action_type::search_case_sensitive;
use action_type::search_count_occurrence;
use action_type::search_with_line_number;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name)?;

    let lines: Vec<String> = match config.option {
        ConfigOptions::CaseInSensitive => {
            search_case_insensitive(&config.query, &file_content)
        }
        ConfigOptions::CaseSensitive => {
            search_case_sensitive(&config.query, &file_content)
        }
        ConfigOptions::CountOccurrence => {
            search_count_occurrence(&config.query, &file_content)
        }
        ConfigOptions::WithLineNumber => {
            search_with_line_number(&config.query, &file_content)
        }
        ConfigOptions::None => vec![],
    };

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
