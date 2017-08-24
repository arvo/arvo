//! # Parser

#[cfg(test)]
pub mod mod_test;

#[macro_use]
pub mod macros;

use super::lexer::{Position, Span, Spanned, Token, Tokens};
use super::ast::*;

//
pub fn parse_expr(tokens: &mut Tokens) -> ParseResult<Expr> {
    try_parse!(
        parse_literal_expr(tokens)
    )
}

//
pub fn parse_paren_expr(tokens: &mut Tokens) -> ParseResult<Expr> {

    // consume the '(' token
    let parenl = match eat_token!(Token::ParenL(..) in tokens) {
        Some(parenl) => parenl,
        _ => return Err(ParseError::new("expected '('", Span::new("", 0, 0, 0, 0))),
    };

    // if the next token is ')' then this is a void expression
    match eat_token!(Token::ParenR(..) in tokens) {
        Some(parenr) => {
            // return the void expression
            return Ok(VoidExpr::new(Span::new_from_positions(
                parenl.span().begin().clone(),
                parenr.span().end().clone()
            )).into());
        }
        _ => (),
    };

    // otherwise parse the inner expression
    let result = parse_expr(tokens);
    match result {
        // expect a closing ')'
        Ok(mut expr) => match eat_token!(Token::ParenR(..) in tokens) {
            Some(parenr) => {
                // adjust the span of the inner expression
                expr.set_span_begin(parenl.span().begin().clone());
                expr.set_span_end(parenr.span().end().clone());
                // return the inner expression
                return Ok(expr);
            }
            None => Err(ParseError::new("expected ')", Span::new("", 0, 0, 0, 0))),
        },
        // or, ignore all tokens until the closing ')'
        Err(..) => match eat_tokens_until!(Token::ParenR(..) in tokens) {
            None => Err(ParseError::new("expected ')'", Span::new("", 0, 0, 0, 0))),
            _ => result,
        },
    }
}

//
pub fn parse_literal_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    try_parse!(
        parse_literal_bool_expr(tokens),
        parse_literal_char_expr(tokens),
        parse_literal_float_expr(tokens),
        parse_literal_integer_expr(tokens),
        parse_literal_string_expr(tokens)
    )
}

//
pub fn parse_literal_bool_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    let token = eat_token!(Token::Bool(..) in tokens);
    match token {
        Some(Token::Bool(val, span, ..)) => Ok(LiteralExpr::Bool(val, span)),
        Some(token) => Err(ParseError::new("expected bool literal", Span::new("", 0, 0, 0, 0))),
        _ => Err(ParseError::new("expected bool literal", Span::new("", 0, 0, 0, 0))),
    }
}

//
pub fn parse_literal_char_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    let token = eat_token!(Token::Char(..) in tokens);
    match token {
        Some(Token::Char(val, span, ..)) => Ok(LiteralExpr::Char(val, span)),
        Some(token) => Err(ParseError::new("expected char literal", Span::new("", 0, 0, 0, 0))),
        _ => Err(ParseError::new("expected char literal", Span::new("", 0, 0, 0, 0))),
    }
}

//
pub fn parse_literal_float_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    let token = eat_token!(Token::Float(..) in tokens);
    match token {
        Some(Token::Float(val, span, ..)) => Ok(LiteralExpr::Float(val, span)),
        Some(token) => Err(ParseError::new("expected float literal", Span::new("", 0, 0, 0, 0))),
        _ => Err(ParseError::new("expected float literal", Span::new("", 0, 0, 0, 0))),
    }
}

//
pub fn parse_literal_integer_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    let token = eat_token!(Token::Int(..) in tokens);
    match token {
        Some(Token::Int(val, span, ..)) => Ok(LiteralExpr::Int(val, span)),
        Some(token) => Err(ParseError::new("expected integer literal", Span::new("", 0, 0, 0, 0))),
        _ => Err(ParseError::new("expected integer literal", Span::new("", 0, 0, 0, 0))),
    }
}

//
pub fn parse_literal_string_expr(tokens: &mut Tokens) -> ParseResult<LiteralExpr> {
    let token = eat_token!(Token::Str(..) in tokens);
    match token {
        Some(Token::Str(val, span, ..)) => Ok(LiteralExpr::Str(val, span)),
        Some(token) => Err(ParseError::new("expected string literal", Span::new("", 0, 0, 0, 0))),
        _ => Err(ParseError::new("expected string literal", Span::new("", 0, 0, 0, 0))),
    }
}

//
pub fn parse_operator_expr(tokens: &mut Tokens) -> ParseResult<OperatorExpr> {
    unimplemented!()
}

//
pub type ParseResult<T> = Result<T, ParseError>;

//
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    message: String,
    span: Span,
}

impl ParseError {
    pub fn new<M, S>(message: M, span: S) -> ParseError
        where M: Into<String>,
              S: Into<Span>
    {
        ParseError {
            message: message.into(),
            span: span.into(),
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}