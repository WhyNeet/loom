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
    assert_eq!(tokens[2], Token::Identifier("a".to_string()));
    assert_eq!(tokens[4], Token::Operator("=".to_string()));
    assert_eq!(tokens[6], Token::Literal(Literal::Number("1".to_string())));
    assert_eq!(tokens[7], Token::Punctuation(';'));
    assert_eq!(tokens[9], Token::Keyword("let".to_string()));
    assert_eq!(tokens[11], Token::Identifier("b".to_string()));
    assert_eq!(tokens[13], Token::Operator("=".to_string()));
    assert_eq!(tokens[15], Token::Literal(Literal::Number("2".to_string())));
    assert_eq!(tokens[16], Token::Punctuation(';'));
    assert_eq!(
        tokens[18],
        Token::Comment("// this is a comment".to_string())
    );
    assert_eq!(tokens[20], Token::Keyword("let".to_string()));
    assert_eq!(tokens[22], Token::Identifier("sum".to_string()));
    assert_eq!(tokens[24], Token::Operator("=".to_string()));
    assert_eq!(tokens[26], Token::Identifier("a".to_string()));
    assert_eq!(tokens[28], Token::Operator("+".to_string()));
    assert_eq!(tokens[30], Token::Identifier("b".to_string()));
    assert_eq!(tokens[32], Token::Operator("/".to_string()));
    assert_eq!(
        tokens[34],
        Token::Literal(Literal::Number("2.5".to_string()))
    );
    assert_eq!(tokens[35], Token::Punctuation(';'));
    assert_eq!(tokens[37], Token::EOF);
}
