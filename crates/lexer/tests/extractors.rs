use common::types::Type;
use lexer::lexer::extractors;

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

#[test]
pub fn punctuation_extractor_works() {
    let string = "012345;7{9}";

    let p1 = extractors::extract_punctuation(&string[6..]);
    assert!(p1.is_some());
    assert_eq!(&p1.unwrap().to_string(), ";");

    let p2 = extractors::extract_punctuation(&string[8..]);
    assert!(p2.is_some());
    assert_eq!(&p2.unwrap().to_string(), "{");

    let p3 = extractors::extract_punctuation(&string[10..]);
    assert!(p3.is_some());
    assert_eq!(&p3.unwrap().to_string(), "}");
}

#[test]
pub fn comment_extrator_works() {
    let string = r#"// this is a comment
    hello /* another comment */ world"#;

    let c1 = extractors::extract_comment(string);
    assert!(c1.is_some());
    assert_eq!(c1.as_ref().unwrap(), "// this is a comment");

    let c2 = extractors::extract_comment(&string[31..]);
    assert!(c2.is_some());
    assert_eq!(c2.as_ref().unwrap(), "/* another comment */");
}

#[test]
pub fn string_extractor_works() {
    let string = r#"let a = "hello, world";"#;

    let s1 = extractors::extract_string(&string[8..]);
    assert_eq!(&s1, "hello, world");
}

#[test]
pub fn type_extractor_works() {
    let string = "u32-=";

    let t1 = extractors::extract_type(string);
    assert!(t1.is_some());
    assert_eq!(t1.unwrap().0, Type::UInt32);
}

#[test]
pub fn identifier_extractor_works() {
    let string = "let x = a + b / 2;";

    let i1 = extractors::extract_identifier(&string[4..]);
    assert_eq!(i1, "x");

    let i2 = extractors::extract_identifier(&string[8..]);
    assert_eq!(i2, "a");

    let i3 = extractors::extract_identifier(&string[12..]);
    assert_eq!(i3, "b");

    let string = "let var;able = 5;";

    let i4 = extractors::extract_identifier(&string[4..]);
    assert_eq!(i4, "var");
}
