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
