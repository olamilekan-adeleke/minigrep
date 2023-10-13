pub struct Config {
    pub query: String,
    pub file_name: String,
    pub option: ConfigOptions,
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
            config_option = ConfigOptions::CaseInSensitive;
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
            "-i" => ConfigOptions::CaseInSensitive,
            "-e" => ConfigOptions::CaseSensitive,
            "-c" => ConfigOptions::CountOccurrence,
            "-n" => ConfigOptions::WithLineNumber,
            _ => ConfigOptions::None,
        }
    }
}

pub enum ConfigOptions {
    CaseSensitive,
    CaseInSensitive,
    CountOccurrence,
    WithLineNumber,
    None,
}
