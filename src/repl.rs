use super::lexer;
use super::parser;

use std::io;
use std::io::Write;

pub fn repl() {

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        input.clear();
        stdin
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        if input == "" {
            break;
        }
        println!("you said {}", input);

        let tokens = lexer::Token::tokenise("", input.as_str());
        let expr = parser::Parser::new(tokens).parse_expr();

        println!("{:#?}", expr);
    }
}