use ::lexer::lexer::Lexer;
use parser::Parser;

#[test]
pub fn parser_works() {
    let input = r#"
      fun square(a: i32, b: i32) -> i32 {
        return a * b;
      }

      fun add(a: i32, b: i32) -> i32 {
        const sum = a + b + { return a * b; };

        if sum > 5 {
          return square(sum);
        } else if sum > 1 && sum < 3 {
          return -1;
        } else {
          return 5;
        }

        let x = 1;
      }
    "#;

    let tokens = Lexer::new().run(input);

    let ast = Parser::new().run(&tokens);

    println!("ast: {ast:?}");
}
