use common::{types::Type, util::traversal};
use lexer::lexer::token::Token;

use crate::ast::{
    declaration::Declaration,
    expression::Expression,
    operation::Operation,
    statement::{LoopStatement, Statement},
    unit::{ASTUnit, Block},
};

use super::parsers;

pub fn parse(tokens: &[Token]) -> (ASTUnit, usize) {
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
                            expression: Box::new(expression),
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

                        ASTUnit::Statement(Statement::Return(vec![expression.0]))
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
                            condition: Box::new(condition),
                            execute: Box::new(block),
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

                        pos += block.len();

                        let (block, _) = parse(block);

                        // an else clause is present
                        let alternative = if tokens[pos] == Token::Keyword("else".to_string()) {
                            pos += 1;
                            let (statement, size) = parse(&tokens[pos..]);

                            pos += size;

                            Some(Box::new(
                                match statement {
                                    ASTUnit::Block(block) => block,
                                    _ => unreachable!(),
                                }
                                .swap_remove(0),
                            ))
                        } else {
                            None
                        };

                        ASTUnit::Statement(Statement::ControlFlow {
                            condition: Box::new(condition),
                            execute: Box::new(block),
                            alternative,
                        })
                    }
                    parsers::Keyword::ControlFlowElse => {
                        todo!()
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
                            pos += 2 + 1;
                            match tokens[pos + 2] {
                                Token::Type(ref ty) => ty.clone(),
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
                            expression: Box::new(expression),
                        })
                    }
                };

                units.push(unit);
            }
            Token::Identifier(_ident) => {
                // this is probably an assignment or function call

                // let unit = match &tokens[pos] {
                //     Token::Operator(op) => {
                //         let operation = Operation::from_str(&op).unwrap();
                //         pos += 1;
                //         let expression = &tokens[pos..(pos
                //             + tokens[pos..]
                //                 .iter()
                //                 .position(|tok| tok == &Token::Punctuation(';'))
                //                 .map(|pos| pos + 1)
                //                 .unwrap_or(tokens.len() - pos))];
                //         pos += expression.len();
                //         let expression = parsers::parse_expression(expression);
                //         ASTUnit::Expression(Expression::BinaryExpression {
                //             left: Box::new(ASTUnit::Expression(Expression::Identifier(
                //                 ident.to_string(),
                //             ))),
                //             right: Box::new(expression.0),
                //             operation,
                //         })
                //     }
                //     token => panic!("not implemented: {token:?}"),
                // };

                let (unit, size) = parsers::parse_expression(&tokens[pos..]);

                pos += size;

                units.push(unit);
            }
            _ => pos += 1,
        }
    }

    (ASTUnit::Block(units), pos)
}
