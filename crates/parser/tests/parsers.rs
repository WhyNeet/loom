use lexer::lexer::lexer;
use parser::parser::parsers;

#[test]
pub fn expression_parser_works() {
    let input = r#"(1 + 2) * 3 > x || x > a"#;

    let tokens = lexer(input);
    let ast = parsers::parse_expression(&tokens);

    println!("{ast:?}");
}
