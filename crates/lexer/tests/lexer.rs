use lexer::lexer::{
    lexer,
    token::{Literal, Token},
};

#[test]
pub fn lexer_works() {
    let code = r#"let a = 1;
let b = 2; // this is a comment
let sum = a + b / 2.5;
"#;

    let tokens = lexer(code);

    assert_eq!(tokens[0], Token::Keyword("let".to_string()));
    assert_eq!(tokens[1], Token::Identifier("a".to_string()));
    assert_eq!(tokens[2], Token::Operator("=".to_string()));
    assert_eq!(tokens[3], Token::Literal(Literal::Number("1".to_string())));
    assert_eq!(tokens[4], Token::Punctuation(';'));
    assert_eq!(tokens[5], Token::Keyword("let".to_string()));
    assert_eq!(tokens[6], Token::Identifier("b".to_string()));
    assert_eq!(tokens[7], Token::Operator("=".to_string()));
    assert_eq!(tokens[8], Token::Literal(Literal::Number("2".to_string())));
    assert_eq!(tokens[9], Token::Punctuation(';'));
    assert_eq!(
        tokens[10],
        Token::Comment("// this is a comment".to_string())
    );
    assert_eq!(tokens[11], Token::Keyword("let".to_string()));
    assert_eq!(tokens[12], Token::Identifier("sum".to_string()));
    assert_eq!(tokens[13], Token::Operator("=".to_string()));
    assert_eq!(tokens[14], Token::Identifier("a".to_string()));
    assert_eq!(tokens[15], Token::Operator("+".to_string()));
    assert_eq!(tokens[16], Token::Identifier("b".to_string()));
    assert_eq!(tokens[17], Token::Operator("/".to_string()));
    assert_eq!(
        tokens[18],
        Token::Literal(Literal::Number("2.5".to_string()))
    );
    assert_eq!(tokens[19], Token::Punctuation(';'));
    assert_eq!(tokens[20], Token::EOF);
}
