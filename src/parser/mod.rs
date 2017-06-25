
/// The different tokens that can appear in Arvo source code.
#[derive(Clone)]
pub enum Token {

    /// Comments.
    Comment,

    /// Identifiers and literals.
    Digit,
    Letter,
    Bool,
    Float,
    Int,
    Identifier,

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
    LeftLambda,

}