use super::*;
use super::super::ast::{LiteralExpr};
use super::super::lexer::{Span, Token};

#[test]
fn parse_literals() {
    assert_eq!(
        parse_literal_bool_expr(&mut vec![
            Token::Bool(true, Span::new("", 1, 1, 1, 4))
        ]), 
        Ok(LiteralExpr::Bool(true, Span::new("", 1, 1, 1, 4)))
    );
    assert_eq!(
        parse_literal_char_expr(&mut vec![
            Token::Char('ҧ', Span::new("", 1, 1, 1, 3))
        ]), 
        Ok(LiteralExpr::Char('ҧ', Span::new("", 1, 1, 1, 3)))
    );
    assert_eq!(
        parse_literal_float_expr(&mut vec![
            Token::Float(3.14, Span::new("", 1, 1, 1, 4))
        ]), 
        Ok(LiteralExpr::Float(3.14, Span::new("", 1, 1, 1, 4)))
    );
    assert_eq!(
        parse_literal_integer_expr(&mut vec![
            Token::Int(42, Span::new("", 1, 1, 1, 4))
        ]), 
        Ok(LiteralExpr::Int(42, Span::new("", 1, 1, 1, 4)))
    );
    assert_eq!(
        parse_literal_string_expr(&mut vec![
            Token::Str("你好".to_string(), Span::new("", 1, 1, 1, 6))
        ]), 
        Ok(LiteralExpr::Str("你好".to_string(), Span::new("", 1, 1, 1, 6)))
    );
}

#[test]
fn parse_paren_exprs() {
    assert_eq!(
        parse_paren_expr(&mut vec![
            Token::ParenL(Span::new("", 1, 1, 1, 1)),
            Token::ParenR(Span::new("", 1, 2, 1, 2))
        ]), 
        Ok(VoidExpr::new(Span::new("", 1, 1, 1, 2)).into())
    );
    assert_eq!(
        parse_paren_expr(&mut vec![
            Token::ParenL(Span::new("", 1, 1, 1, 1)),
            Token::Int(1, Span::new("", 1, 2, 1, 2)),
            Token::ParenR(Span::new("", 1, 3, 1, 3))
        ]), 
        Ok(LiteralExpr::Int(1, Span::new("", 1, 1, 1, 3)).into())
    );
}

