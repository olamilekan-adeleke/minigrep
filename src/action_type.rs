pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let result = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|e| e.to_string())
        .collect();

    result
}

pub fn search_case_sensitive(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|e| e.to_string())
        .collect()
}

pub fn search_with_line_number(query: &str, contents: &str) -> Vec<String> {
    let result = search_case_insensitive(query, contents);
    result
        .into_iter()
        .enumerate()
        .map(|(index, e)| format!("{}:{}", index, e))
        .collect::<Vec<String>>()
}

pub fn search_count_occurrence(query: &str, contents: &str) -> Vec<String> {
    let lines = search_case_insensitive(query, contents);
    let count = lines.into_iter().count();
    vec![count.to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_insensitive_works() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_search_case_sensitive_works() {
        let query = "RUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let result: Vec<&str> = Vec::new();
        assert_eq!(result, search_case_sensitive(query, contents));
    }
}
