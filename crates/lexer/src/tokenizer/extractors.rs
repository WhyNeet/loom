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
    for keyword in KEYWORDS {
        if input.starts_with(keyword) {
            return Some(keyword.to_string());
        }
    }

    None
}
