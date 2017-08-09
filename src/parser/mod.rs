//! # Parser

#[cfg(test)]
pub mod mod_test;

use super::lexer::Tokens;
use super::ast::Ast;

pub struct Parser {
    ast: Ast,
}

impl Parser {

    pub fn parse(tokens: Tokens) -> Ast {
        let mut parser = Parser::new();
        parser.ast()
    }

    pub fn new() -> Parser {
        Parser {
            ast: Ast::Nil
        }
    }

    fn parse_module_decl(&mut self, tokens: Tokens) {
    }

    fn parse_function_decl(&mut self, tokens: Tokens) {
    }

    fn parse_type_decl(&mut self, tokens: Tokens) {
    }

    fn parse_expr(&mut self, tokens: Tokens) {
    }

    fn ast(self) -> Ast {
        self.ast
    }

}