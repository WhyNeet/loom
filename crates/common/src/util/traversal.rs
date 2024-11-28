pub fn traverse_till_root_par<T: PartialEq>(input: &[T], parentheses: (T, T)) -> Option<usize> {
    let mut stack = Vec::new();
    let mut pos = 0;

    while pos < input.len() {
        let token = &input[pos];

        if token != &parentheses.0 && token != &parentheses.1 {
            pos += 1;
            continue;
        }

        if stack.is_empty() || stack.last().unwrap() == &token {
            stack.push(token)
        } else if stack.last().unwrap() == &&parentheses.0 && token == &parentheses.1 {
            stack.pop();
            if stack.is_empty() {
                return Some(pos);
            }
        } else if stack.last().unwrap() == &&parentheses.1 && token == &parentheses.0 {
            stack.push(token)
        }

        pos += 1;
    }

    None
}
