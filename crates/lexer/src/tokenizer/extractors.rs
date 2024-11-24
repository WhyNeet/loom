use super::keywords::KEYWORDS;

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

pub fn extract_operator(input: &str) -> Option<String> {
    if ["+", "-", "*", "/"].contains(&&input[0..1]) {
        Some(input[0..1].to_string())
    } else {
        None
    }
}
