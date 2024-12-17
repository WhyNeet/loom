use std::{mem, rc::Rc};

use ast::{
    declaration::{Declaration, VariableDeclarationKeyword},
    expression::Expression,
    literal::Literal,
    operation::Operation,
    statement::{LoopStatement, Statement},
    unit::ASTUnit,
    AbstractSyntaxTree,
};
use common::{
    constants::keywords::{
        DECLARATION_CONSTANT, DECLARATION_FUNCTION, DECLARATION_VARIABLE, STATEMENT_ELSE,
        STATEMENT_IF, STATEMENT_RETURN, STATEMENT_WHILE,
    },
    types::Type,
    util::traversal,
};
use lexer::lexer::token::Token;

pub mod ast;

pub enum Keyword {
    FunctionDeclaration,
    VariableDeclaration(VariableDeclarationKeyword),
    ControlFlowIf,
    ControlFlowElse,
    Return,
    While,
}

pub enum RecognizableStructure {
    Block((usize, usize)),
    FunctionInvokation((usize, usize)),
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, input: &[Token]) -> AbstractSyntaxTree {
        AbstractSyntaxTree::new(self.run_internal(input).0)
    }

    fn run_internal(&self, tokens: &[Token]) -> (ASTUnit, usize) {
        let mut pos = 0;
        let (tokens, offset) = if tokens.first().is_some()
            && tokens.first().unwrap() == &Token::Punctuation('{')
            && tokens.last().is_some()
            && tokens.last().unwrap() == &Token::Punctuation('}')
        {
            (&tokens[1..(tokens.len() - 1)], 2)
        } else {
            (tokens, 0)
        };

        let mut units = vec![];

        while pos < tokens.len() {
            let token = &tokens[pos];

            match token {
                Token::Identifier(_) | Token::Literal(_) => {
                    let (expression, size) = self.parse_expression(&tokens[pos..]);
                    pos += size;

                    let unit = if pos >= tokens.len() || tokens[pos] != Token::Punctuation(';') {
                        // if the expression is in the end of the code block
                        ASTUnit::Statement(Statement::ImplicitReturn(Rc::new(expression)))
                    } else {
                        expression
                    };

                    units.push(Rc::new(unit));
                }
                Token::Keyword(keyword) => {
                    let keyword = self.parse_keyword(&keyword).unwrap();

                    match keyword {
                        Keyword::Return => {
                            pos += 1;

                            let expression = &tokens[pos..];

                            let (expression, size) = self.parse_expression(expression);

                            // expression size + ";"
                            pos += size + 1;

                            units.push(Rc::new(expression));
                        }
                        Keyword::VariableDeclaration(keyword) => {
                            pos += 1;

                            let identifier = match &tokens[pos] {
                                Token::Identifier(identifier) => identifier,
                                other => panic!("expected variable identifier, got: {other:?}"),
                            }
                            .clone();

                            pos += 1;

                            let expression = &tokens[pos..];

                            let (expression, size) = self.run_internal(expression);

                            // expression size + ";"
                            pos += size + 1;

                            units.push(Rc::new(ASTUnit::Declaration(
                                Declaration::VariableDeclaration {
                                    keyword,
                                    identifier,
                                    expression: Rc::new(expression),
                                },
                            )));
                        }
                        Keyword::FunctionDeclaration => {
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

                            let (expression, _) =
                                self.run_internal(&tokens[pos..(pos + block_end_offset)]);

                            pos += block_end_offset;

                            units.push(Rc::new(ASTUnit::Declaration(
                                Declaration::FunctionDeclaration {
                                    identifier,
                                    parameters,
                                    return_type,
                                    expression: Rc::new(expression),
                                },
                            )));
                        }
                        Keyword::While => {
                            pos += 1;

                            let condition = &tokens[pos..];
                            let (condition, size) = self.parse_expression(condition);

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

                            let (block, _) = self.run_internal(block);

                            units.push(Rc::new(ASTUnit::Statement(Statement::Loop(
                                LoopStatement::While {
                                    condition: Rc::new(condition),
                                    execute: Rc::new(block),
                                },
                            ))));
                        }
                        Keyword::ControlFlowIf => {
                            pos += 1;
                            let (control_flow, size) = self.parse_control_flow(&tokens[pos..]);
                            pos += size;
                            units.push(Rc::new(control_flow));
                        }
                        Keyword::ControlFlowElse => unreachable!(),
                    };
                }
                Token::Punctuation('{') => {
                    let (unit, size) = self.parse_expression(
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

        (ASTUnit::Block(units), pos + offset)
    }

    fn parse_control_flow(&self, tokens: &[Token]) -> (ASTUnit, usize) {
        let mut offset = 0;

        let mut end = 0;

        while end < tokens.len() && tokens[end] != Token::Keyword("else".to_string()) {
            end += if tokens[end] == Token::Punctuation('{') {
                traversal::traverse_till_root_par(
                    &tokens[end..],
                    (Token::Punctuation('{'), Token::Punctuation('}')),
                )
                .unwrap_or(tokens.len() - end)
            } else {
                1
            };
        }

        let (condition, size) = self.parse_expression(&tokens[..end]);

        offset += size;

        let block = &tokens[offset
            ..traversal::traverse_till_root_par(
                &tokens[offset..],
                (Token::Punctuation('{'), Token::Punctuation('}')),
            )
            .map(|pos| offset + pos + 1)
            .unwrap_or(tokens.len())];

        let (block, size) = self.run_internal(block);

        offset += size;

        let alternative = if offset < tokens.len() {
            if tokens[offset] == Token::Keyword("else".to_string()) {
                offset += 1;
                let (alternative, size) = self.run_internal(&tokens[offset..]);
                offset += size;
                Some(Rc::new(alternative))
            } else {
                None
            }
        } else {
            None
        };

        (
            ASTUnit::Statement(Statement::ControlFlow {
                condition: Rc::new(condition),
                execute: Rc::new(block),
                alternative,
            }),
            offset,
        )
    }

    fn parse_expression(&self, expression: &[Token]) -> (ASTUnit, usize) {
        if expression.len() == 0 {
            return (
                ASTUnit::Expression(Expression::Literal(Literal::Int32(0))),
                0,
            );
        }

        let mut size = 0;

        let expression = if expression[0] == Token::Punctuation('(')
            && expression.last().unwrap() == &Token::Punctuation(')')
        {
            size += 2;
            &expression[1..(expression.len() - 1)]
        } else {
            expression
        };

        if let Some(structure) = self.recognize_structure(expression) {
            match structure {
                RecognizableStructure::Block((start, end)) => {
                    let (unit, size) = self.run_internal(&expression[(start + 1)..(end - 1)]);
                    return (unit, size + 2);
                }
                RecognizableStructure::FunctionInvokation((start, end)) => {
                    let identifier = expression[start].as_identifier().unwrap().to_string();
                    let params: Vec<Rc<ASTUnit>> = expression[(start + 1)..end]
                        .split(|tok| tok == &Token::Punctuation(','))
                        .map(|expr| self.parse_expression(expr))
                        .map(|(expr, _)| Rc::new(expr))
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
            expression
                .iter()
                .enumerate()
                .fold(None, |acc, (idx, tok)| match tok {
                    Token::Operator(op) => {
                        if parentheses_count != 0 || braces_count != 0 || semicolon_count != 0 {
                            return acc;
                        }

                        let operation = Operation::from_str(op).unwrap();
                        if acc.is_none() || acc.as_ref().unwrap().1.gt(&operation) {
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
            let (left, right) = expression.split_at(idx);
            // ignore lowest operator
            let right = &right[1..];

            let (left, left_size) = self.parse_expression(left);
            let (right, right_size) = self.parse_expression(right);

            // left + right + operator
            size += left_size + right_size + 1;

            (
                ASTUnit::Expression(Expression::BinaryExpression {
                    left: Rc::new(left),
                    right: Rc::new(right),
                    operation: lowest,
                }),
                size,
            )
        } else {
            let literal_or_ident = expression
                .iter()
                .position(|tok| match tok {
                    Token::Literal(_) | Token::Identifier(_) => true,
                    _ => false,
                })
                .unwrap();

            size += literal_or_ident + 1;

            let literal_or_ident = &expression[literal_or_ident];

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

    fn recognize_structure(&self, input: &[Token]) -> Option<RecognizableStructure> {
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
        } else if mem::discriminant(&input[0])
            == mem::discriminant(&Token::Identifier("".to_string()))
            && input.len() > 1
            && &input[1] == &Token::Punctuation('(')
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

    fn parse_keyword(&self, keyword: &str) -> Option<Keyword> {
        match keyword {
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
}
