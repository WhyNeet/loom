use super::keywords::KEYWORDS;
use common::types::Type;

pub fn extract_number(input: &str) -> String {
    let mut chars = input.chars().peekable();
    let mut number = String::new();

    while chars.peek().is_some() {
        let char = chars.next().unwrap();

        if char.is_ascii_digit() {
            number.push(char);
        } else if char == '.' && chars.peek().is_some() && chars.peek().unwrap().is_ascii_digit() {
            number.push(char);
        } else {
            break;
        }
    }

    number
}

pub fn extract_keyword(input: &str) -> Option<String> {
    let split = input.split_once(' ');
    let string = split.map(|(left, _)| left).unwrap_or(input);

    if KEYWORDS.contains(&string) {
        Some(string.to_string())
    } else {
        None
    }
}

const OPERATORS: &[&str] = &["+", "-", "*", "/", "=", ">=", "<=", "<", ">", "==", "!="];

pub fn extract_operator(input: &str) -> Option<String> {
    if input.len() > 1 && OPERATORS.contains(&&input[0..2]) {
        Some(input[0..2].to_string())
    } else if OPERATORS.contains(&&input[0..1]) {
        Some(input[0..1].to_string())
    } else {
        None
    }
}

const PUNCTUATION: &[&str] = &["{", "}", ";", "(", ")", ".", ",", ":"];

pub fn extract_punctuation(input: &str) -> Option<char> {
    if PUNCTUATION.contains(&&input[0..1]) {
        Some(input.as_bytes()[0] as char)
    } else {
        None
    }
}

pub fn extract_comment(input: &str) -> Option<String> {
    if input.starts_with("//") {
        let newline_pos = input.find('\n');
        Some(input[..newline_pos.unwrap_or(input.len())].to_string())
    } else if input.starts_with("/*") {
        let comment_end = input.find("*/");
        if comment_end.is_none() {
            None
        } else {
            Some(input[..comment_end.map(|end| end + 2).unwrap()].to_string())
        }
    } else {
        None
    }
}

pub fn extract_string(input: &str) -> String {
    let string_end = &input[1..].find('"').map(|idx| idx + 1);
    input[1..string_end.unwrap_or(input.len())].to_string()
}

pub fn extract_type(input: &str) -> Option<(Type, usize)> {
    let input = &input[..input
        .chars()
        .position(|c| !c.is_ascii_alphanumeric())
        .unwrap_or(input.len())];

    Type::from(input).map(|t| (t, input.len()))
}

pub fn extract_identifier(input: &str) -> String {
    let mut ident = String::new();
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        if !char.is_ascii_alphabetic() {
            break;
        }

        ident.push(char);
    }

    ident
}
