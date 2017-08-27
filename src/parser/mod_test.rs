use super::*;
use super::super::ast::*;
use super::super::lexer::*;

#[test]
fn parse_literal_bool_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Bool(true, Span::new("", 1, 1, 1, 4))
        ]).parse_literal_bool_expr(),
        LiteralExpr::Bool(true, Span::new("", 1, 1, 1, 4))
    );
}

#[test]
fn parse_literal_char_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Char('ҧ', Span::new("", 1, 1, 1, 3))
        ]).parse_literal_char_expr(), 
        LiteralExpr::Char('ҧ', Span::new("", 1, 1, 1, 3))
    );
}

#[test]
fn parse_literal_float_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Float(3.14, Span::new("", 1, 1, 1, 4))
        ]).parse_literal_float_expr(), 
        LiteralExpr::Float(3.14, Span::new("", 1, 1, 1, 4))
    );
}

#[test]
fn parse_literal_int_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Int(42, Span::new("", 1, 1, 1, 4))
        ]).parse_literal_int_expr(), 
        LiteralExpr::Int(42, Span::new("", 1, 1, 1, 4))
    );
}

#[test]
fn parse_literal_str_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Str("你好".to_string(), Span::new("", 1, 1, 1, 6))
        ]).parse_literal_str_expr(), 
        LiteralExpr::Str("你好".to_string(), Span::new("", 1, 1, 1, 6))
    );
}

#[test]
fn parse_binary_operator_expr() {
    assert_eq!(
        Parser::new(vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Add(Span::new("", 1, 2, 1, 2)),
            Token::Int(2, Span::new("", 1, 3, 1, 3)),
        ]).parse_expr(),
        BinaryOperatorExpr::new(
            Operator::Add,
            LiteralExpr::Int(1, Span::new("", 1, 1, 1, 1)),
            LiteralExpr::Int(2, Span::new("", 1, 3, 1, 3))
        ).into()
    );
}

#[test]
fn parse_binary_operator_expr_with_precedence() {
    assert_eq!(
        Parser::new(vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Mul(Span::new("", 1, 2, 1, 2)),
            Token::Int(2, Span::new("", 1, 3, 1, 3)),
            Token::Add(Span::new("", 1, 4, 1, 4)),
            Token::Int(3, Span::new("", 1, 5, 1, 5)),
        ]).parse_expr(),
        BinaryOperatorExpr::new(
            Operator::Add,
            BinaryOperatorExpr::new(
                Operator::Mul,
                LiteralExpr::Int(1, Span::new("", 1, 1, 1, 1)),
                LiteralExpr::Int(2, Span::new("", 1, 3, 1, 3)),
            ),
            LiteralExpr::Int(3, Span::new("", 1, 5, 1, 5)),
        ).into()
    );
    assert_eq!(
        Parser::new(vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Add(Span::new("", 1, 2, 1, 2)),
            Token::Int(2, Span::new("", 1, 3, 1, 3)),
            Token::Mul(Span::new("", 1, 4, 1, 4)),
            Token::Int(3, Span::new("", 1, 5, 1, 5)),
        ]).parse_expr(),
        BinaryOperatorExpr::new(
            Operator::Add,
            LiteralExpr::Int(1, Span::new("", 1, 1, 1, 1)),
            BinaryOperatorExpr::new(
                Operator::Mul,
                LiteralExpr::Int(2, Span::new("", 1, 3, 1, 3)),
                LiteralExpr::Int(3, Span::new("", 1, 5, 1, 5)),
            ),
        ).into()
    );
    assert_eq!(
        Parser::new(vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Mul(Span::new("", 1, 2, 1, 2)),
            Token::Int(2, Span::new("", 1, 3, 1, 3)),
            Token::Add(Span::new("", 1, 4, 1, 4)),
            Token::Int(3, Span::new("", 1, 5, 1, 5)),
            Token::Mul(Span::new("", 1, 6, 1, 6)),
            Token::Int(4, Span::new("", 1, 7, 1, 7)),
        ]).parse_expr(),
        BinaryOperatorExpr::new(
            Operator::Add,
            BinaryOperatorExpr::new(
                Operator::Mul,
                LiteralExpr::Int(1, Span::new("", 1, 1, 1, 1)),
                LiteralExpr::Int(2, Span::new("", 1, 3, 1, 3)),
            ),
            BinaryOperatorExpr::new(
                Operator::Mul,
                LiteralExpr::Int(3, Span::new("", 1, 5, 1, 5)),
                LiteralExpr::Int(4, Span::new("", 1, 7, 1, 7)),
            ),
        ).into()
    );
}