use lexer::tokenizer::extractors;

#[test]
pub fn number_extractor_works() {
    let string = "1234.56 + 33";

    let n1 = extractors::extract_number(string);
    assert_eq!(&n1, "1234.56");

    let n2 = extractors::extract_number(&string[(n1.len() + 3)..]);
    assert_eq!(&n2, "33");
}
