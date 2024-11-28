use common::util::traversal;
use lexer::lexer::token::Token;

use crate::ast::{ASTUnit, Block, Declaration, Statement};

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

                        let identifier = match tokens[pos + 1] {
                            Token::Identifier(ref ident) => ident.clone(),
                            _ => panic!("expected identfier"),
                        };

                        pos += 1;

                        match tokens[pos] {
                            Token::Punctuation('=') => (),
                            _ => panic!("expected '=' after variable identifier"),
                        }

                        pos += 1;

                        let expression = &tokens[pos..tokens[pos..]
                            .iter()
                            .position(|tok| match tok {
                                Token::Punctuation(';') => true,
                                _ => false,
                            })
                            .unwrap()];

                        pos += expression.len();

                        let expression = parse(expression);

                        ASTUnit::Declaration(Declaration::VariableDeclaration {
                            keyword: decl,
                            identifier,
                            expression,
                        })
                    }
                    parsers::Keyword::Return => {
                        pos += 1;

                        let expression = &tokens[pos..tokens
                            .iter()
                            .skip(pos)
                            .position(|tok| match tok {
                                Token::Punctuation(';') => true,
                                _ => false,
                            })
                            .unwrap()];
                        let expression = parse(expression);

                        pos += expression.len();

                        ASTUnit::Statement(Statement::Return(expression))
                    }
                    parsers::Keyword::While => {
                        // let condition = &tokens[pos..tokens
                        //     .iter()
                        //     .skip(pos)
                        //     .position(|tok| {
                        //         match tok { Token::Punctuation('{') }
                        //     })
                        //     .unwrap()];

                        // pos += condition.len();

                        // let condition = parse(condition);

                        // let block =
                        panic!("while not implemented for now")
                    }
                    parsers::Keyword::ControlFlowIf | parsers::Keyword::ControlFlowElse => {
                        panic!("if-else not implemented for now")
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
            _ => pos += 1,
        }
    }

    units
}
