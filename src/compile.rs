use super::lexer::Token;
use super::parser::Parser;

pub fn compile(input: &str) {
    let tokens = Token::tokenise(input);
    let ast = Parser::parse(tokens);
}