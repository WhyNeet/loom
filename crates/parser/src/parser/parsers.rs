use std::cmp::Ordering;

use common::constants::keywords::{
    DECLARATION_CONSTANT, DECLARATION_FUNCTION, DECLARATION_VARIABLE, STATEMENT_ELSE, STATEMENT_IF,
    STATEMENT_RETURN, STATEMENT_WHILE,
};
use lexer::lexer::token::Token;

use crate::ast::{self, ASTUnit, Expression, Operation, VariableDeclarationKeyword};

pub enum Keyword {
    FunctionDeclaration,
    VariableDeclaration(VariableDeclarationKeyword),
    ControlFlowIf,
    ControlFlowElse,
    Return,
    While,
}

pub fn parse_keyword(input: &str) -> Option<Keyword> {
    match input {
        DECLARATION_CONSTANT => Some(Keyword::VariableDeclaration(
            VariableDeclarationKeyword::Const,
        )),
        DECLARATION_FUNCTION => Some(Keyword::FunctionDeclaration),
        DECLARATION_VARIABLE => Some(Keyword::VariableDeclaration(
            VariableDeclarationKeyword::Const,
        )),
        STATEMENT_IF => Some(Keyword::ControlFlowIf),
        STATEMENT_ELSE => Some(Keyword::ControlFlowElse),
        STATEMENT_WHILE => Some(Keyword::While),
        STATEMENT_RETURN => Some(Keyword::Return),
        _ => None,
    }
}

pub fn parse_expression(input: &[Token]) -> ASTUnit {
    let input = if input[0] == Token::Punctuation('(')
        && input.last().unwrap() == &Token::Punctuation(')')
    {
        &input[1..(input.len() - 1)]
    } else {
        input
    };

    let mut parentheses_count = 0;

    let lowest_precedence: Option<(usize, Operation)> =
        input
            .iter()
            .enumerate()
            .fold(None, |acc, (idx, tok)| match tok {
                Token::Operator(op) => {
                    if parentheses_count != 0 {
                        return acc;
                    }

                    let operation = Operation::from_str(op).unwrap();
                    if acc.is_none() || acc.as_ref().unwrap().1.ge(&operation) {
                        Some((idx, operation))
                    } else {
                        acc
                    }
                }
                Token::Punctuation('(') => {
                    parentheses_count += 1;
                    acc
                }
                Token::Punctuation(')') => {
                    parentheses_count -= 1;
                    acc
                }
                _ => acc,
            });

    if let Some((idx, lowest)) = lowest_precedence {
        let (left, right) = input.split_at(idx);
        // ignore lowest operator
        let right = &right[1..];
        ASTUnit::Expression(Expression::BinaryExpression {
            left: vec![parse_expression(left)],
            right: vec![parse_expression(right)],
            operation: lowest,
        })
    } else {
        let literal_or_ident = input.iter().find(|tok| match tok {
            Token::Literal(_) | Token::Identifier(_) => true,
            _ => false,
        });

        ASTUnit::Expression(match literal_or_ident.unwrap() {
            Token::Literal(literal) => {
                Expression::Literal(ast::Literal::from_literal_token(literal))
            }
            Token::Identifier(ident) => Expression::Identifier(ident.clone()),
            _ => unreachable!(),
        })
    }
}
