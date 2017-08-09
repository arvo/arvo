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
fn tokenise_main_function() {
    assert_eq!(
        Token::tokenise(r#"fn main() -> {}"#),
        vec![
            Token::Func,
            Token::Ident("main".to_string()),
            Token::ParenL,
            Token::ParenR,
            Token::LambdaR,
            Token::BraceL,
            Token::BraceR
        ]
    );
}