pub fn truncate_too_long_string(
    string: impl Into<String>,
    max_length: usize,
    postfix: &str,
) -> String {
    let text = string.into();
    if text.len() > max_length {
        return text[..max_length].to_string() + postfix;
    }
    text
}

pub fn cut_string_by_delimiter_keep_right(
    string: impl Into<String>,
    delimiter: &str,
) -> String {
    let text = string.into();
    let split = text.split(delimiter);
    split.last().unwrap().to_string()
}

pub fn trim_left_slashes(text: impl Into<String>) -> String {
    let mut result = text.into();
    while result.starts_with('/') {
        result = result[1..].to_string();
    }
    result
}

pub fn parse_comma_separated_list(string: impl Into<String>) -> Vec<String> {
    let text = string.into();
    let split = text.split(',');
    let result: Vec<String> = split.map(|s| s.trim().to_string()).collect();
    result
}
