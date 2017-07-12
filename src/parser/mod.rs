//! # Parser

#[cfg!(test)]
mod test;

use regex::Regex;

/// The different tokens that can appear in Arvo source code.
#[derive(Clone)]
pub enum Token {
    /// Comments.
    Comment(String),

    /// Identifiers and literals.
    Char(char),
    Bool(bool),
    Float(f64),
    Int(i64),
    Identifier(String),
    Str(String),

    /// Whitespace.
    Newline,
    Whitespace,

    /// Keywords and symbols.
    As,
    Else,
    Extern,
    Expose,
    For,
    Func,
    If,
    Import,
    In,
    Module,
    Mut,
    Ref,
    Type,
    SemiColon,
    Colon,
    Comma,
    Dot,
    DotDot,
    LeftLambda,
    RightLambda,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Pipe,

    /// Operators.
    Assign,
    Add,
    Div,
    Mul,
    Sub,
    AddEq,
    DivEq,
    MulEq,
    SubEq,
    Equal,
    LessThan,
    LessThanEq,
    GreaterThan,
    GreaterThanEq,
    And,
    Or,
}

impl Token {
    pub fn regex(&self) -> Regex {
        match *self {
            Token::Comment(..) => Regex::new(r"\/\/(.)*\n").unwrap(),
            Token::Char(..) => unimplemented!(),
            Token::Bool(..) => Regex::new(r"(true|false)").unwrap(),
            Token::Float(..) => Regex::new(r"\d*\.\d+").unwrap(),
            Token::Int(..) => Regex::new(r"\d+").unwrap(),
            Token::Identifier(..) => Regex::new(r"[a-zA-Z]([a-zA-Z]|\d)+").unwrap(),
            Token::Str(..) => unimplemented!(),
            Token::Newline => Regex::new(r"\n").unwrap(),
            Token::Whitespace => Regex::new(r"[ \t\r\f]").unwrap(),
            Token::As => Regex::new(r"as").unwrap(),
            Token::Else => Regex::new(r"else").unwrap(),
            Token::Extern => Regex::new(r"extern").unwrap(),
            Token::Expose => Regex::new(r"expose").unwrap(),
            Token::For => Regex::new(r"for").unwrap(),
            Token::Func => Regex::new(r"fn").unwrap(),
            Token::If => Regex::new(r"if").unwrap(),
            Token::Import => Regex::new(r"import").unwrap(),
            Token::In => Regex::new(r"in").unwrap(),
            Token::Module => Regex::new(r"module").unwrap(),
            Token::Mut => Regex::new(r"mut").unwrap(),
            Token::Ref => Regex::new(r"ref").unwrap(),
            Token::Type => Regex::new(r"type").unwrap(),
            Token::SemiColon => Regex::new(r";").unwrap(),
            Token::Colon => Regex::new(r":").unwrap(),
            Token::Comma => Regex::new(r",").unwrap(),
            Token::Dot => Regex::new(r"\.").unwrap(),
            Token::DotDot => Regex::new(r"\.\.").unwrap(),
            Token::LeftLambda => Regex::new(r"<-").unwrap(),
            Token::RightLambda => Regex::new(r"->").unwrap(),
            Token::LeftParen => Regex::new(r"\(").unwrap(),
            Token::RightParen => Regex::new(r"\)").unwrap(),
            Token::LeftBracket => Regex::new(r"\[").unwrap(),
            Token::RightBracket => Regex::new(r"\]").unwrap(),
            Token::LeftBrace => Regex::new(r"{").unwrap(),
            Token::RightBrace => Regex::new(r"}").unwrap(),
            Token::Pipe => Regex::new(r"\|").unwrap(),   
            Token::Assign => Regex::new(r":=").unwrap(),
            Token::Add => Regex::new(r"\+").unwrap(),
            Token::Div => Regex::new(r"\/").unwrap(),
            Token::Mul => Regex::new(r"\*").unwrap(),
            Token::Sub => Regex::new(r"-").unwrap(),
            Token::AddEq => Regex::new(r"\+=").unwrap(),
            Token::DivEq => Regex::new(r"\/=").unwrap(),
            Token::MulEq => Regex::new(r"\*=").unwrap(),
            Token::SubEq => Regex::new(r"-=").unwrap(),
            Token::Equal => Regex::new(r"=").unwrap(),
            Token::LessThan => Regex::new(r"<").unwrap(),
            Token::LessThanEq => Regex::new(r"<=").unwrap(),
            Token::GreaterThan => Regex::new(r">").unwrap(),
            Token::GreaterThanEq => Regex::new(r">=").unwrap(),
            Token::And => Regex::new(r"&&").unwrap(),
            Token::Or => Regex::new(r"\|\|").unwrap(),
        }
    }
}