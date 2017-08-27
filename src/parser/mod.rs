//! # Parser

#[cfg(test)]
pub mod mod_test;

#[macro_use]
pub mod macros;

use super::lexer::{Position, Span, Spanned, Token, Tokens, Precedence};
use super::ast::*;

pub struct Parser {
    cursor: usize,
    tokens: Tokens,
    errors: Vec<String>,
}

impl Parser {

    pub fn new(tokens: Tokens) -> Parser {
        Parser {
            cursor: 0,
            tokens: tokens,
            errors: Vec::new(),
        }
    }

    pub fn peek_token(&self, lookahead: usize) -> Option<Token> {
        if self.cursor + lookahead < self.tokens.len() {
            Some(self.tokens[self.cursor + lookahead].clone())
        } else {
            None
        }
    }

    pub fn current_token(&self) -> Option<Token> {
        if self.cursor < self.tokens.len() {
            Some(self.tokens[self.cursor].clone())
        } else {
            None
        }
    }

    pub fn next_token(&mut self) {
        self.cursor += 1;
    }

    pub fn parse_expr(&mut self) -> Expr {
        let lhs_expr = self.parse_unary_expr();
        self.parse_binary_expr(lhs_expr)
    }

    pub fn parse_binary_expr(&mut self, lhs_expr: Expr) -> Expr {

        // Get the next token.
        let token = match self.current_token() {
            Some(token) => token,
            // Having no next token should finish the parsing of the binary
            // expression.
            None => return lhs_expr,
        };
        let prec = token.precedence();

        // Parse the binary operator.
        let operator = match token {
            Token::Add(..) => Operator::Add,
            Token::Div(..) => Operator::Div,
            Token::Mul(..) => Operator::Mul,
            Token::Sub(..) => Operator::Sub,
            _ => return lhs_expr,
        };

        // We have now decided that this token can be parsed by this function
        // so we can consume it.
        self.next_token();

        // Parse the next part of this binary expression.
        let rhs_expr = self.parse_unary_expr();

        match self.current_token() {
            Some(next_token) => if prec > next_token.precedence() {
                // The next token is of a lower precedence so this binary
                // expression is should be parsed first, as the left hand side
                // of the next.
                self.parse_binary_expr(BinaryOperatorExpr::new(
                    operator,
                    lhs_expr,
                    rhs_expr,
                ).into())
            } else {
                // The next token is of a higher precedence so the right hand
                // side of this expression is actually the left hand side of 
                // the next binary expression.
                BinaryOperatorExpr::new(
                    operator,
                    lhs_expr,
                    self.parse_binary_expr(rhs_expr),
                ).into()
            },
            // There are no more parts of the expression.
            None => BinaryOperatorExpr::new(
                operator,
                lhs_expr,
                rhs_expr,
            ).into(),
        }        
    }

    pub fn parse_unary_expr(&mut self) -> Expr {
        let token = self.current_token();
        match token {
            Some(Token::ParenL(..)) => self.parse_paren_expr(),
            Some(Token::Bool(..)) => self.parse_literal_bool_expr().into(),
            Some(Token::Char(..)) => self.parse_literal_char_expr().into(),
            Some(Token::Float(..)) => self.parse_literal_float_expr().into(),
            Some(Token::Int(..)) => self.parse_literal_int_expr().into(),
            Some(Token::Str(..)) => self.parse_literal_str_expr().into(),
            _ => unimplemented!(),
        }
    }

    pub fn parse_paren_expr(&mut self) -> Expr {

        // Eat the left parenthesis.
        let token = self.current_token();
        match token {
            Some(Token::ParenL(..)) => self.next_token(),
            _ => unimplemented!(),
        };

        // Parse the inner expression.
        let expr = self.parse_expr();

        // Eat the right parenthesis.
        let token = self.current_token();
        match token {
            Some(Token::ParenR(..)) => self.next_token(),
            _ => unimplemented!(),
        };

        expr
    }

    //
    pub fn parse_literal_bool_expr(&mut self) -> LiteralExpr {
        let token = self.current_token();
        self.next_token();
        match token {
            Some(Token::Bool(val, span, ..)) => LiteralExpr::Bool(val, span),
            _ => unimplemented!(),
        }
    }

    //
    pub fn parse_literal_char_expr(&mut self) -> LiteralExpr {
        let token = self.current_token();
        self.next_token();
        match token {
            Some(Token::Char(val, span, ..)) => LiteralExpr::Char(val, span),
            _ => unimplemented!(),
        }
    }

    //
    pub fn parse_literal_float_expr(&mut self) -> LiteralExpr {
        let token = self.current_token();
        self.next_token();
        match token {
            Some(Token::Float(val, span, ..)) => LiteralExpr::Float(val, span),
            _ => unimplemented!(),
        }
    }

    //
    pub fn parse_literal_int_expr(&mut self) -> LiteralExpr {
        let token = self.current_token();
        self.next_token();
        match token {
            Some(Token::Int(val, span, ..)) => LiteralExpr::Int(val, span),
            _ => unimplemented!(),
        }
    }

    //
    pub fn parse_literal_str_expr(&mut self) -> LiteralExpr {
        let token = self.current_token();
        self.next_token();
        match token {
            Some(Token::Str(val, span, ..)) => LiteralExpr::Str(val, span),
            _ => unimplemented!(),
        }
    }

}