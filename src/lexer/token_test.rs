use super::token::{Token};

#[test]
fn tokenise_literals() {
    assert_eq!(
        Token::tokenise(r#"true"#),
        vec![Token::Bool(true)]
    );
    assert_eq!(
        Token::tokenise(r#"false"#),
        vec![Token::Bool(false)]
    );
    assert_eq!(
        Token::tokenise(r#"'ä'"#),
        vec![Token::Char('ä')]
    );
    assert_eq!(
        Token::tokenise(r#"'\u00E4'"#),
        vec![Token::Char('ä')]
    );
    assert_eq!(
        Token::tokenise(r#"3.14"#),
        vec![Token::Float(3.14)]
    );
    assert_eq!(
        Token::tokenise(r#"42"#),
        vec![Token::Int(42)]
    );
    assert_eq!(
        Token::tokenise(r#""Arvo Pärt""#),
        vec![Token::Str("Arvo Pärt".to_string())]
    );
}

#[test]
fn tokenise_operators() {
    assert_eq!(
        Token::tokenise(r#"1 + 2"#),
        vec![Token::Int(1), Token::Add, Token::Int(2)]
    );
    assert_eq!(
        Token::tokenise(r#"1 / 2"#),
        vec![Token::Int(1), Token::Div, Token::Int(2)]
    );
    assert_eq!(
        Token::tokenise(r#"1 * 2"#),
        vec![Token::Int(1), Token::Mul, Token::Int(2)]
    );
    assert_eq!(
        Token::tokenise(r#"1 - 2"#),
        vec![Token::Int(1), Token::Sub, Token::Int(2)]
    );
}

#[test]
fn tokenise_function() {
    assert_eq!(
        Token::tokenise(r#"fn main() void -> {}"#),
        vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::ParenL,
            Token::ParenR,
            Token::Ident("void".to_string()),
            Token::LambdaR,
            Token::BraceL,
            Token::BraceR
        ]
    );
}

#[test]
fn tokenise_type_enum() {
    assert_eq!(
        Token::tokenise(r#"type Option a = Some a | Nil"#),
        vec![
            Token::Type,
            Token::Ident("Option".to_string()),
            Token::Ident("a".to_string()),
            Token::Equal,
            Token::Ident("Some".to_string()),
            Token::Ident("a".to_string()),
            Token::Pipe,
            Token::Ident("Nil".to_string())
        ]
    );
}

#[test]
fn tokenise_type_struct() {
    assert_eq!(
        Token::tokenise(r#"type Point { x f64, y f64 }"#),
        vec![
            Token::Type,
            Token::Ident("Point".to_string()),
            Token::BraceL,
            Token::Ident("x".to_string()),
            Token::Ident("f64".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::Ident("f64".to_string()),
            Token::BraceR
        ]
    );
}

#[test]
fn tokenise_if() {
    assert_eq!(
        Token::tokenise(r#"if true { 1 } else { 0 }"#),
        vec![
            Token::If,
            Token::Bool(true),
            Token::BraceL,
            Token::Int(1),
            Token::BraceR,
            Token::Else,
            Token::BraceL,
            Token::Int(0),
            Token::BraceR
        ]
    );
}

#[test]
fn tokenise_for() {
    assert_eq!(
        Token::tokenise(r#"for i in 1 .. 100 { }"#),
        vec![
            Token::For,
            Token::Ident("i".to_string()),
            Token::In,
            Token::Int(1),
            Token::DotDot,
            Token::Int(100),
            Token::BraceL,
            Token::BraceR
        ]
    );
}

#[test]
fn tokenise_writeln() {
    assert_eq!(
        Token::tokenise(r#"writeln("Hello, Arvo Pärt")"#),
        vec![
            Token::Ident("writeln".to_string()),
            Token::ParenL,
            Token::Str("Hello, Arvo Pärt".to_string()),
            Token::ParenR
        ]
    );
}