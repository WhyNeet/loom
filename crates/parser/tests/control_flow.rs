use lexer::lexer::Lexer;
use parser::Parser;

#[test]
pub fn control_flow_parsing_works() {
    let input = r#"
    let x = if a > b {
      a
    };
  "#;

    let tokens = Lexer::new().run(input);
    let ast = Parser::new().run(&tokens);

    println!("{ast:?}");
}
