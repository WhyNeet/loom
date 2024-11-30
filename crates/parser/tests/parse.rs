use lexer::lexer;
use parser::parser;

#[test]
pub fn parser_works() {
    let input = r#"
      fun add(a: i32, b: i32) -> i32 {
        const sum = a + b;

        if sum > 5 {
          return sum;
        } else if sum > 1 && sum < 3 {
          return -1;
        } else {
          return 5;
        }
      }
    "#;

    let tokens = lexer::lexer(input);

    let (ast, _) = parser::parse(&tokens);

    println!("ast: {ast:?}");
}
