use common::util::traversal;
use lexer::lexer::token::Token;

use crate::ast::{
    declaration::Declaration,
    expression::Expression,
    operation::Operation,
    statement::{LoopStatement, Statement},
    unit::{ASTUnit, Block},
};

use super::parsers;

pub fn parse(tokens: &[Token]) -> Block {
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

                        let expression = &tokens[pos..(pos
                            + tokens[pos..]
                                .iter()
                                .position(|tok| match tok {
                                    Token::Punctuation(';') => true,
                                    _ => false,
                                })
                                .unwrap())];

                        pos += expression.len();

                        let expression = parsers::parse_expression(expression);

                        ASTUnit::Declaration(Declaration::VariableDeclaration {
                            keyword: decl,
                            identifier,
                            expression: vec![expression.0],
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
                                .unwrap())];
                        pos += expression.len();

                        let expression = parsers::parse_expression(expression);

                        ASTUnit::Statement(Statement::Return(vec![expression.0]))
                    }
                    parsers::Keyword::While => {
                        let condition = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .unwrap_or(
                                tokens
                                    .iter()
                                    .position(|tok| tok == &Token::Punctuation('{'))
                                    .unwrap(),
                            ))];

                        pos += condition.len();

                        let condition = parse(condition);

                        let block = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .unwrap_or(
                                tokens
                                    .iter()
                                    .position(|tok| tok == &Token::Punctuation('}'))
                                    .unwrap(),
                            ))];

                        pos += block.len();

                        let block = parse(block);

                        ASTUnit::Statement(Statement::Loop(LoopStatement::While {
                            condition,
                            execute: block,
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
                            .unwrap_or(tokens.len() - pos))];

                        let (condition, condition_size) = parsers::parse_expression(condition);

                        pos += condition_size;

                        let block = &tokens[pos..(pos
                            + traversal::traverse_till_root_par(
                                &tokens[pos..],
                                (Token::Punctuation('{'), Token::Punctuation('}')),
                            )
                            .unwrap_or(
                                tokens
                                    .iter()
                                    .position(|tok| tok == &Token::Punctuation('}'))
                                    .unwrap(),
                            ))];

                        pos += block.len();

                        let block = parse(block);

                        ASTUnit::Statement(Statement::ControlFlow {
                            condition: vec![condition],
                            execute: block,
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
                        pos += 1;

                        let parameters = tokens[pos..(pos + args_end_offset - 1)]
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
                            .collect();

                        pos += args_end_offset;

                        // pos + "->".len()
                        let return_type = match tokens[pos + 2] {
                            Token::Type(ref ty) => ty.clone(),
                            _ => panic!("expected return type"),
                        };

                        // "->" + "type"
                        pos += 2 + 1;

                        let block_end_offset = traversal::traverse_till_root_par(
                            &tokens[pos..],
                            (Token::Punctuation('{'), Token::Punctuation('}')),
                        )
                        .unwrap();

                        let expression = parse(&tokens[pos..(pos + block_end_offset)]);

                        pos += block_end_offset;

                        ASTUnit::Declaration(Declaration::FunctionDeclaration {
                            identifier,
                            parameters,
                            return_type,
                            expression,
                        })
                    }
                };

                units.push(unit);
            }
            Token::Identifier(ident) => {
                // this is probably an assignment or function call

                pos += 1;

                let unit = match &tokens[pos] {
                    Token::Operator(op) => {
                        let operation = Operation::from_str(&op).unwrap();
                        pos += 1;
                        let expression = &tokens[pos..(pos
                            + tokens[pos..]
                                .iter()
                                .position(|tok| tok == &Token::Punctuation(';'))
                                .unwrap_or(tokens.len() - pos))];
                        pos += expression.len();
                        let expression = parsers::parse_expression(expression);
                        ASTUnit::Expression(Expression::BinaryExpression {
                            left: vec![ASTUnit::Expression(Expression::Identifier(
                                ident.to_string(),
                            ))],
                            right: vec![expression.0],
                            operation,
                        })
                    }
                    _ => panic!("not implemented"),
                };

                units.push(unit);
            }
            _ => pos += 1,
        }
    }

    units
}
