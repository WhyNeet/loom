use std::rc::Rc;

use common::{types::Type, util::traversal};
use lexer::lexer::token::Token;

use crate::ast::{
    declaration::Declaration,
    statement::{LoopStatement, Statement},
    unit::ASTUnit,
};

use super::parsers;

pub fn parse(tokens: &[Token]) -> (ASTUnit, usize) {
    let mut add = 0;

    let tokens = if tokens.first().is_some()
        && tokens.first().unwrap() == &Token::Punctuation('{')
        && tokens.last().is_some()
        && tokens.last().unwrap() == &Token::Punctuation('}')
    {
        add += 2;
        &tokens[1..(tokens.len() - 1)]
    } else {
        tokens
    };

    let mut units = Vec::new();
    let mut pos = 0;

    while pos < tokens.len() {
        let token = &tokens[pos];

        match token {
            Token::Keyword(keyword) => {
                let keyword = parsers::parse_keyword(&keyword).unwrap();

                let unit = match keyword {
                    parsers::Keyword::VariableDeclaration(decl) => {
                        // let or const
                        pos += 1;

                        let identifier = match tokens[pos] {
                            Token::Identifier(ref ident) => ident.clone(),
                            _ => panic!("expected identifier"),
                        };

                        pos += 1;

                        match tokens[pos] {
                            Token::Operator(ref op) => {
                                if op != "=" {
                                    panic!("expected '=' after variable identifier")
                                }
                            }
                            _ => panic!("expected '=' after variable identifier"),
                        }

                        // assignment operator
                        pos += 1;

                        let expression = &tokens[pos..];

                        let (expression, size) = parsers::parse_expression(expression);
                        pos += size;

                        ASTUnit::Declaration(Declaration::VariableDeclaration {
                            keyword: decl,
                            identifier,
                            expression: Rc::new(expression),
                        })
                    }
                    parsers::Keyword::Return => {
                        pos += 1;

                        let expression = &tokens[pos..(pos
                            + tokens[pos..]
                                .iter()
                                .position(|tok| match tok {
                                    Token::Punctuation(';') => true,
                                    _ => false,
                                })
                                .map(|pos| pos + 1)
                                .unwrap())];
                        pos += expression.len();

                        let expression = parsers::parse_expression(expression);

                        ASTUnit::Statement(Statement::Return(Rc::new(expression.0)))
                    }
                    parsers::Keyword::While => {
                        pos += 1;

                        let condition = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .map(|pos| pos + 1)
                            .unwrap_or(tokens.len() - pos))];
                        let (condition, size) = parsers::parse_expression(condition);

                        pos += size;

                        let block = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .map(|pos| pos + 1)
                            .unwrap_or(
                                tokens
                                    .iter()
                                    .position(|tok| tok == &Token::Punctuation('}'))
                                    .map(|pos| pos + 1)
                                    .unwrap(),
                            ))];

                        pos += block.len();

                        let (block, _) = parse(block);

                        ASTUnit::Statement(Statement::Loop(LoopStatement::While {
                            condition: Rc::new(condition),
                            execute: Rc::new(block),
                        }))
                    }
                    parsers::Keyword::ControlFlowIf => {
                        // if keyword
                        pos += 1;

                        let condition = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .map(|pos| pos + 1)
                            .unwrap_or(tokens.len() - pos))];

                        let (condition, condition_size) = parsers::parse_expression(condition);
                        pos += condition_size;

                        let block = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .map(|pos| pos + 1)
                            .unwrap_or(
                                tokens
                                    .iter()
                                    .position(|tok| tok == &Token::Punctuation('}'))
                                    .map(|pos| pos + 1)
                                    .unwrap(),
                            ))];

                        let (block, size) = parse(block);
                        pos += size;

                        // an else clause is present
                        let alternative = if pos < tokens.len()
                            && tokens[pos] == Token::Keyword("else".to_string())
                        {
                            pos += 1;
                            let mut alt_tokens = &tokens[pos..(pos
                                + traversal::traverse_till_root_par(
                                    &tokens[pos..],
                                    (Token::Punctuation('{'), Token::Punctuation('}')),
                                )
                                .map(|pos| pos + 1)
                                .unwrap_or(
                                    tokens
                                        .iter()
                                        .position(|tok| tok == &Token::Punctuation('}'))
                                        .map(|pos| pos + 1)
                                        .unwrap(),
                                ))];

                            // it works
                            // dont touch it
                            while (pos + alt_tokens.len()) < tokens.len()
                                && tokens[pos + alt_tokens.len()]
                                    == Token::Keyword("else".to_string())
                            {
                                alt_tokens = &tokens[pos..(pos
                                    + alt_tokens.len()
                                    + traversal::traverse_till_root_par(
                                        &tokens[(pos + alt_tokens.len())..],
                                        (Token::Punctuation('{'), Token::Punctuation('}')),
                                    )
                                    .map(|pos| pos + 1)
                                    .unwrap())];
                            }

                            let (statement, size) = parse(alt_tokens);

                            pos += size;

                            Some(statement)
                        } else {
                            None
                        };

                        ASTUnit::Statement(Statement::ControlFlow {
                            condition: Rc::new(condition),
                            execute: Rc::new(block),
                            alternative: alternative.map(|alt| Rc::new(alt)),
                        })
                    }
                    parsers::Keyword::ControlFlowElse => {
                        unreachable!()
                    }
                    parsers::Keyword::FunctionDeclaration => {
                        // fun keyword
                        pos += 1;

                        let identifier = match tokens[pos] {
                            Token::Identifier(ref ident) => ident.clone(),
                            _ => panic!("expected function identifier"),
                        };
                        // ident
                        pos += 1;

                        let args_end_offset = traversal::traverse_till_root_par(
                            &tokens[pos..],
                            (Token::Punctuation('('), Token::Punctuation(')')),
                        )
                        .unwrap();
                        // (
                        pos += 1;

                        let parameters = if tokens[pos] == Token::Punctuation(')') {
                            vec![]
                        } else {
                            tokens[pos..(pos + args_end_offset - 1)]
                                .split(|tok| tok.eq(&Token::Punctuation(',')))
                                .map(|param| (param[0].clone(), param[2].clone()))
                                .map(|(ident, ty)| {
                                    (
                                        match ident {
                                            Token::Identifier(ident) => ident,
                                            _ => panic!("expected identifier"),
                                        },
                                        match ty {
                                            Token::Type(ty) => ty,
                                            _ => panic!("expected type"),
                                        },
                                    )
                                })
                                .collect()
                        };

                        pos += args_end_offset;

                        // pos + "->".len()
                        let return_type = if tokens[pos] == Token::Operator("-".to_string())
                            && tokens[pos + 1] == Token::Operator(">".to_string())
                        {
                            // "->" + "type"
                            match tokens[pos + 2] {
                                Token::Type(ref ty) => {
                                    pos += 2 + 1;
                                    ty.clone()
                                }
                                _ => panic!("expected return type"),
                            }
                        } else {
                            Type::Void
                        };

                        let block_end_offset = traversal::traverse_till_root_par(
                            &tokens[pos..],
                            (Token::Punctuation('{'), Token::Punctuation('}')),
                        )
                        .map(|pos| pos + 1)
                        .unwrap();

                        let (expression, _) = parse(&tokens[pos..(pos + block_end_offset)]);

                        pos += block_end_offset;

                        ASTUnit::Declaration(Declaration::FunctionDeclaration {
                            identifier,
                            parameters,
                            return_type,
                            expression: Rc::new(expression),
                        })
                    }
                };

                units.push(Rc::new(unit));
            }
            Token::Identifier(_) | Token::Literal(_) => {
                let (expression, size) = parsers::parse_expression(&tokens[pos..]);
                pos += size;

                let unit = if pos >= tokens.len() || tokens[pos] != Token::Punctuation(';') {
                    // if the expression is in the end of the code block
                    ASTUnit::Statement(Statement::ImplicitReturn(Rc::new(expression)))
                } else {
                    expression
                };

                units.push(Rc::new(unit));
            }
            Token::Punctuation('{') => {
                let (unit, size) = parsers::parse_expression(
                    &tokens[pos..(pos
                        + traversal::traverse_till_root_par(
                            &tokens[pos..],
                            (Token::Punctuation('{'), Token::Punctuation('}')),
                        )
                        .map(|pos| pos + 1)
                        .unwrap())],
                );

                pos += size;

                units.push(Rc::new(unit));
            }
            _ => pos += 1,
        }
    }

    (ASTUnit::Block(units), pos + add)
}
