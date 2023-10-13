use std::error::Error;
use std::fmt::Write;
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
        let mut colored_text = String::new();
        let query = &config.query;

        for word in line.split(" ") {
            if word.contains(query) {
                let start_index = word.find(query).unwrap();
                let end_index = start_index + query.len();

                colored_text.write_str(&word[..start_index]).unwrap();
                colored_text.write_str("\x1b[31m").unwrap();
                colored_text
                    .write_str(&word[start_index..end_index])
                    .unwrap();
                colored_text.write_str("\x1b[0m").unwrap();
                colored_text.write_str(&word[end_index..]).unwrap();
            } else {
                colored_text.write_str(word).unwrap();
            }

            colored_text.write_str(" ").unwrap();
        }

        colored_text.to_string();
        println!("{}", colored_text);
    }

    Ok(())
}
