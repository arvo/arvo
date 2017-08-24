use super::span::{Span};
use super::token::{Token};

#[test]
fn tokenise_literals() {
    assert_eq!(
        Token::tokenise("", "true"),
        vec![Token::Bool(true, Span::new("", 1, 1, 1, 4))]
    );
    assert_eq!(
        Token::tokenise("", "false"),
        vec![Token::Bool(false, Span::new("", 1, 1, 1, 5))]
    );
    assert_eq!(
        Token::tokenise("", "'ä'"),
        vec![Token::Char('ä', Span::new("", 1, 1, 1, 3))]
    );
    assert_eq!(
        Token::tokenise("", "'\\u00E4'"),
        vec![Token::Char('ä', Span::new("", 1, 1, 1, 8))]
    );
    assert_eq!(
        Token::tokenise("", "3.14"),
        vec![Token::Float(3.14, Span::new("", 1, 1, 1, 4))]
    );
    assert_eq!(
        Token::tokenise("", "42"),
        vec![Token::Int(42, Span::new("", 1, 1, 1, 2))]
    );
    assert_eq!(
        Token::tokenise("", "\"Arvo Pärt\""),
        vec![Token::Str("Arvo Pärt".to_string(), Span::new("", 1, 1, 1, 11))]
    );
}

#[test]
fn tokenise_operators() {
    assert_eq!(
        Token::tokenise("", "1 + 2"),
        vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Add(Span::new("", 1, 3, 1, 3)),
            Token::Int(2, Span::new("", 1, 5, 1, 5))]
    );
    assert_eq!(
        Token::tokenise("", "1 / 2"),
        vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Div(Span::new("", 1, 3, 1, 3)),
            Token::Int(2, Span::new("", 1, 5, 1, 5))]
    );
    assert_eq!(
        Token::tokenise("", "1 * 2"),
        vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Mul(Span::new("", 1, 3, 1, 3)),
            Token::Int(2, Span::new("", 1, 5, 1, 5))]
    );
    assert_eq!(
        Token::tokenise("", "1 - 2"),
        vec![
            Token::Int(1, Span::new("", 1, 1, 1, 1)),
            Token::Sub(Span::new("", 1, 3, 1, 3)),
            Token::Int(2, Span::new("", 1, 5, 1, 5))]
    );
}

#[test]
fn tokenise_function() {
    assert_eq!(
        Token::tokenise("", "fn main() void -> {\n}"),
        vec![
            Token::Func(Span::new("", 1, 1, 1, 2)),
            Token::Ident("main".to_string(), Span::new("", 1, 4, 1, 7)),
            Token::ParenL(Span::new("", 1, 8, 1, 8)),
            Token::ParenR(Span::new("", 1, 9, 1, 9)),
            Token::Ident("void".to_string(), Span::new("", 1, 11, 1, 14)),
            Token::LambdaR(Span::new("", 1, 16, 1, 17)),
            Token::BraceL(Span::new("", 1, 19, 1, 19)),
            Token::WhitespaceNewline(Span::new("", 1, 20, 1, 20)),
            Token::BraceR(Span::new("", 2, 1, 2, 1))
        ]
    );
}

#[test]
fn tokenise_type_enum() {
    assert_eq!(
        Token::tokenise("", "type Option a = Some a | Nil"),
        vec![
            Token::Type(Span::new("", 1, 1, 1, 4)),
            Token::Ident("Option".to_string(), Span::new("", 1, 6, 1, 11)),
            Token::Ident("a".to_string(), Span::new("", 1, 13, 1, 13)),
            Token::Equal(Span::new("", 1, 15, 1, 15)),
            Token::Ident("Some".to_string(), Span::new("", 1, 17, 1, 20)),
            Token::Ident("a".to_string(), Span::new("", 1, 22, 1, 22)),
            Token::Pipe(Span::new("", 1, 24, 1, 24)),
            Token::Ident("Nil".to_string(), Span::new("", 1, 26, 1, 28))
        ]
    );
}

#[test]
fn tokenise_type_struct() {
    assert_eq!(
        Token::tokenise("", "type Point { x f64, y f64 }"),
        vec![
            Token::Type(Span::new("", 1, 1, 1, 4)),
            Token::Ident("Point".to_string(), Span::new("", 1, 6, 1, 10)),
            Token::BraceL(Span::new("", 1, 12, 1, 12)),
            Token::Ident("x".to_string(), Span::new("", 1, 14, 1, 14)),
            Token::Ident("f64".to_string(), Span::new("", 1, 16, 1, 18)),
            Token::Comma(Span::new("", 1, 19, 1, 19)),
            Token::Ident("y".to_string(), Span::new("", 1, 21, 1, 21)),
            Token::Ident("f64".to_string(), Span::new("", 1, 23, 1, 25)),
            Token::BraceR(Span::new("", 1, 27, 1, 27))
        ]
    );
}

#[test]
fn tokenise_if() {
    assert_eq!(
        Token::tokenise("", "if true {\n  1\n} else {\n  0\n}"),
        vec![
            Token::If(Span::new("", 1, 1, 1, 2)),
            Token::Bool(true, Span::new("", 1, 4, 1, 7)),
            Token::BraceL(Span::new("", 1, 9, 1, 9)),
            Token::WhitespaceNewline(Span::new("", 1, 10, 1, 10)),
            Token::Int(1, Span::new("", 2, 3, 2, 3)),
            Token::WhitespaceNewline(Span::new("", 2, 4, 2, 4)),
            Token::BraceR(Span::new("", 3, 1, 3, 1)),
            Token::Else(Span::new("", 3, 3, 3, 6)),
            Token::BraceL(Span::new("", 3, 8, 3, 8)),
            Token::WhitespaceNewline(Span::new("", 3, 9, 3, 9)),
            Token::Int(0, Span::new("", 4, 3, 4, 3)),
            Token::WhitespaceNewline(Span::new("", 4, 4, 4, 4)),
            Token::BraceR(Span::new("", 5, 1, 5, 1))
        ]
    );
}

#[test]
fn tokenise_for() {
    assert_eq!(
        Token::tokenise("", "for i in 1 .. 100 {\n}"),
        vec![
            Token::For(Span::new("", 1, 1, 1, 3)),
            Token::Ident("i".to_string(), Span::new("", 1, 5, 1, 5)),
            Token::In(Span::new("", 1, 7, 1, 8)),
            Token::Int(1, Span::new("", 1, 10, 1, 10)),
            Token::DotDot(Span::new("", 1, 12, 1, 13)),
            Token::Int(100, Span::new("", 1, 15, 1, 17)),
            Token::BraceL(Span::new("", 1, 19, 1, 19)),
            Token::WhitespaceNewline(Span::new("", 1, 20, 1, 20)),
            Token::BraceR(Span::new("", 2, 1, 2, 1))
        ]
    );
}

#[test]
fn tokenise_writeln() {
    assert_eq!(
        Token::tokenise("", "writeln(\"Hello, Arvo Pärt\")"),
        vec![
            Token::Ident("writeln".to_string(), Span::new("", 1, 1, 1, 7)),
            Token::ParenL(Span::new("", 1, 8, 1, 8)),
            Token::Str("Hello, Arvo Pärt".to_string(), Span::new("", 1, 9, 1, 26)),
            Token::ParenR(Span::new("", 1, 27, 1, 27))
        ]
    );
}