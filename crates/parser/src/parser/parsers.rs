use std::mem;

use common::{
    constants::keywords::{
        DECLARATION_CONSTANT, DECLARATION_FUNCTION, DECLARATION_VARIABLE, STATEMENT_ELSE,
        STATEMENT_IF, STATEMENT_RETURN, STATEMENT_WHILE,
    },
    util::traversal,
};
use lexer::lexer::token::Token;

use crate::ast::{
    declaration::VariableDeclarationKeyword, expression::Expression, literal::Literal,
    operation::Operation, unit::ASTUnit,
};

use super::parse;

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
            VariableDeclarationKeyword::Let,
        )),
        STATEMENT_IF => Some(Keyword::ControlFlowIf),
        STATEMENT_ELSE => Some(Keyword::ControlFlowElse),
        STATEMENT_WHILE => Some(Keyword::While),
        STATEMENT_RETURN => Some(Keyword::Return),
        _ => None,
    }
}

/// Returns an AST unit and expression length in tokens
pub fn parse_expression(input: &[Token]) -> (ASTUnit, usize) {
    if input.len() == 0 {
        return (
            ASTUnit::Expression(Expression::Literal(Literal::Int32(0))),
            0,
        );
    }

    let mut size = 0;

    let input = if input[0] == Token::Punctuation('(')
        && input.last().unwrap() == &Token::Punctuation(')')
    {
        size += 2;
        &input[1..(input.len() - 1)]
    } else {
        input
    };

    if let Some(structure) = recognize_structure(input) {
        match structure {
            RecognizableStructure::Block((start, end)) => {
                println!("=> parse block");
                let (unit, size) = parse(&input[(start + 1)..(end - 1)]);
                return (ASTUnit::Block(unit), size);
            }
            RecognizableStructure::FunctionInvokation((start, end)) => {
                println!("=> parse function invokation");
                let identifier = input[start].as_identifier().unwrap().to_string();
                let params: Vec<ASTUnit> = input[(start + 1)..end]
                    .split(|tok| tok == &Token::Punctuation(','))
                    .map(|expr| parse_expression(expr))
                    .map(|(expr, _)| expr)
                    .collect();

                return (
                    ASTUnit::Expression(Expression::FunctionInvokation {
                        function_name: identifier,
                        parameters: params,
                    }),
                    end - start,
                );
            }
        }
    }

    let mut parentheses_count = 0;
    let mut braces_count = 0;
    let mut semicolon_count = 0;

    let lowest_precedence: Option<(usize, Operation)> =
        input
            .iter()
            .enumerate()
            .fold(None, |acc, (idx, tok)| match tok {
                Token::Operator(op) => {
                    if parentheses_count != 0 || braces_count != 0 || semicolon_count != 0 {
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
                Token::Punctuation('{') => {
                    braces_count += 1;
                    acc
                }
                Token::Punctuation('}') => {
                    braces_count -= 1;
                    acc
                }
                Token::Punctuation(';') => {
                    if parentheses_count != 0 || braces_count != 0 {
                        return acc;
                    }
                    semicolon_count += 1;
                    acc
                }
                _ => acc,
            });

    if let Some((idx, lowest)) = lowest_precedence {
        let (left, right) = input.split_at(idx);
        // ignore lowest operator
        let right = &right[1..];

        let (left, left_size) = parse_expression(left);
        let (right, right_size) = parse_expression(right);

        // left + right + operator
        size += left_size + right_size + 1;

        (
            ASTUnit::Expression(Expression::BinaryExpression {
                left: vec![left],
                right: vec![right],
                operation: lowest,
            }),
            size,
        )
    } else {
        let literal_or_ident = input
            .iter()
            .position(|tok| match tok {
                Token::Literal(_) | Token::Identifier(_) => true,
                _ => false,
            })
            .unwrap();

        size += literal_or_ident + 1;

        let literal_or_ident = &input[literal_or_ident];

        (
            ASTUnit::Expression(match literal_or_ident {
                Token::Literal(literal) => {
                    Expression::Literal(Literal::from_literal_token(literal))
                }
                Token::Identifier(ident) => Expression::Identifier(ident.clone()),
                _ => unreachable!(),
            }),
            size,
        )
    }
}

pub enum RecognizableStructure {
    Block((usize, usize)),
    FunctionInvokation((usize, usize)),
}

pub fn recognize_structure(input: &[Token]) -> Option<RecognizableStructure> {
    if input[0] == Token::Punctuation('{') {
        let end = traversal::traverse_till_root_par(
            input,
            (Token::Punctuation('{'), Token::Punctuation('}')),
        )
        .map(|pos| pos + 1);

        if end.is_none() {
            return None;
        }

        let end = end.unwrap();

        Some(RecognizableStructure::Block((0, end)))
    } else if mem::discriminant(&input[0]) == mem::discriminant(&Token::Identifier("".to_string()))
        && input.len() > 1
        && mem::discriminant(&input[1]) == mem::discriminant(&Token::Punctuation('('))
    {
        let end = traversal::traverse_till_root_par(
            input,
            (Token::Punctuation('('), Token::Punctuation(')')),
        )
        .map(|pos| pos + 1);

        if end.is_none() {
            return None;
        }

        let end = end.unwrap();

        Some(RecognizableStructure::FunctionInvokation((0, end)))
    } else {
        None
    }
}
