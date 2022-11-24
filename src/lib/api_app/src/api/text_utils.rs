/**
 * Split text with a separator, keep the right-most part.
 */
pub fn split_text_keep_right(text: &str, separator: &str) -> String {
    let split = text.split(separator);
    let parts: Vec<&str> = split.collect();
    return parts[parts.len() - 1].to_string();
}
