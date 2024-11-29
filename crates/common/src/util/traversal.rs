pub fn traverse_till_root_par<T: PartialEq>(input: &[T], parentheses: (T, T)) -> Option<usize> {
    let mut count = 0;
    let mut pos = 0;

    while pos < input.len() {
        let token = &input[pos];

        if token != &parentheses.0 && token != &parentheses.1 {
            pos += 1;
            continue;
        }

        if token == &parentheses.1 {
            count -= 1;
            if count == 0 {
                return Some(pos);
            }
        } else if token == &parentheses.0 {
            count += 1;
        }

        pos += 1;
    }

    None
}
