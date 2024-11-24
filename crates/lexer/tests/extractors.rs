use lexer::tokenizer::extractors;

#[test]
pub fn number_extractor_works() {
    let string = "1234.56 + 33";

    let n1 = extractors::extract_number(string);
    assert_eq!(&n1, "1234.56");

    let n2 = extractors::extract_number(&string[(n1.len() + 3)..]);
    assert_eq!(&n2, "33");
}

#[test]
pub fn keyword_extractor_works() {
    let string = "if a > b { return a; } else { return b; }";

    let k1 = extractors::extract_keyword(string);
    assert!(k1.is_some());
    assert_eq!(k1.as_ref().unwrap(), "if");

    let k2 = extractors::extract_keyword(&string[11..]);
    assert!(k2.is_some());
    assert_eq!(k2.as_ref().unwrap(), "return");

    let k3 = extractors::extract_keyword(&string[23..]);
    assert!(k3.is_some());
    assert_eq!(k3.as_ref().unwrap(), "else");
}

#[test]
pub fn operation_extractor_works() {
    let string = "1234.56 + 33 - 5 / 6 * 2";

    let o1 = extractors::extract_operator(&string[8..]);
    assert!(o1.is_some());
    assert_eq!(o1.as_ref().unwrap(), "+");

    let o2 = extractors::extract_operator(&string[13..]);
    assert!(o2.is_some());
    assert_eq!(o2.as_ref().unwrap(), "-");

    let o3 = extractors::extract_operator(&string[17..]);
    assert!(o3.is_some());
    assert_eq!(o3.as_ref().unwrap(), "/");

    let o4 = extractors::extract_operator(&string[21..]);
    assert!(o4.is_some());
    assert_eq!(o4.as_ref().unwrap(), "*");
}
