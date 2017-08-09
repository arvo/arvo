//! The Token module defines the different lexical tokens in the Arvo
//! programming language, and defines some basic operations on these tokens.

// External crate dependencies
use regex;

use std::char;
use std::fmt;
use std::str::FromStr;

/// A Token represents a lexical token in the Arvo programming language. It
/// includes the values of literals, identifiers, and comments.
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Reserved keywords
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

    // Operators
    Add,
    AddEq,
    And,
    Assign,
    Div,
    DivEq,
    Equal,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    Mul,
    MulEq,
    Or,
    PushPop,
    Sub,
    SubEq,

    // Symbols
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Colon,
    Comma,
    Dot,
    DotDot,
    LambdaR,
    ParenL,
    ParenR,
    SemiColon,
    Whitespace,

    // Literals
    Char(char),
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(String),

    // Idents
    Ident(String),

    // Comments
    Comment(String),
}

impl Token {
    pub fn tokenise(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        for capture in Token::regex().captures_iter(input) {
            let token = 

                // Reserved keywords
                if capture.name("As").is_some() {
                    Token::As
                } else if capture.name("Else").is_some() {
                    Token::Else
                } else if capture.name("Expose").is_some() {
                    Token::Expose
                } else if capture.name("Extern").is_some() {
                    Token::Extern
                } else if capture.name("For").is_some() {
                    Token::For
                } else if capture.name("Func").is_some() {
                    Token::Func
                } else if capture.name("If").is_some() {
                    Token::If
                } else if capture.name("Import").is_some() {
                    Token::Import
                } else if capture.name("Module").is_some() {
                    Token::Module
                } else if capture.name("Mut").is_some() {
                    Token::Mut
                } else if capture.name("Ref").is_some() {
                    Token::Ref
                } else if capture.name("Type").is_some() {
                    Token::Type
                }

                // Symbols
                else if capture.name("BraceL").is_some() {
                    Token::BraceL
                } else if capture.name("BraceR").is_some() {
                    Token::BraceR
                } else if capture.name("BracketL").is_some() {
                    Token::BracketL
                } else if capture.name("BracketR").is_some() {
                    Token::BracketR
                } else if capture.name("Colon").is_some() {
                    Token::Colon
                } else if capture.name("Comma").is_some() {
                    Token::Comma
                } else if capture.name("Dot").is_some() {
                    Token::Dot
                } else if capture.name("DotDot").is_some() {
                    Token::DotDot
                } else if capture.name("LambdaR").is_some() {
                    Token::LambdaR
                } else if capture.name("ParenL").is_some() {
                    Token::ParenL
                } else if capture.name("ParenR").is_some() {
                    Token::ParenR
                } else if capture.name("SemiColon").is_some() {
                    Token::SemiColon
                } else if capture.name("Whitespace").is_some() {
                    Token::Whitespace
                }
                
                // Operators
                else if capture.name("Add").is_some() {
                    Token::Add
                } else if capture.name("AddEq").is_some() {
                    Token::AddEq
                } else if capture.name("And").is_some() {
                    Token::And
                } else if capture.name("Assign").is_some() {
                    Token::Assign
                } else if capture.name("Div").is_some() {
                    Token::Div
                } else if capture.name("DivEq").is_some() {
                    Token::DivEq
                } else if capture.name("Equal").is_some() {
                    Token::Equal
                } else if capture.name("GreaterThan").is_some() {
                    Token::GreaterThan
                } else if capture.name("GreaterThanEq").is_some() {
                    Token::GreaterThanEq
                } else if capture.name("LessThan").is_some() {
                    Token::LessThan
                } else if capture.name("LessThanEq").is_some() {
                    Token::LessThanEq
                } else if capture.name("Mul").is_some() {
                    Token::Mul
                } else if capture.name("MulEq").is_some() {
                    Token::MulEq
                } else if capture.name("Or").is_some() {
                    Token::Or
                } else if capture.name("PushPop").is_some() {
                    Token::PushPop
                } else if capture.name("Sub").is_some() {
                    Token::Sub
                } else if capture.name("SubEq").is_some() {
                    Token::SubEq
                }

                // Literals
                else if capture.name("Bool").is_some() {
                    Token::Bool(bool::from_str(capture.name("Bool").unwrap()).unwrap())
                } else if capture.name("Char").is_some() {
                    let string = capture.name("Char").unwrap();
                    let utf_regex = regex::Regex::from_str(r"\\u(([0-9]|[ABCDEF]){4})").unwrap();
                    Token::Char(
                        if utf_regex.is_match(string) {
                            let utf_regex_code = utf_regex.captures_iter(string).next().unwrap();
                            let utf_code = u32::from_str_radix(&utf_regex_code[1], 16).unwrap();
                            char::from_u32(utf_code).unwrap()
                        } else {
                            string.chars().next().unwrap()
                        })
                } else if capture.name("Float").is_some() {
                    Token::Float(f64::from_str(capture.name("Float").unwrap()).unwrap())
                } else if capture.name("Int").is_some() {
                    Token::Int(i64::from_str(capture.name("Int").unwrap()).unwrap())
                } else if capture.name("Str").is_some() {
                    Token::Str(capture.name("Str").unwrap().to_string())
                }

                // Idents
                else if capture.name("Ident").is_some() {
                    Token::Ident(capture.name("Ident").unwrap().to_string())
                }

                // Comments
                else if capture.name("Comment").is_some() {
                    Token::Comment(capture.name("Comment").unwrap().to_string())
                }
                
                // Other
                else {
                    panic!("unexpected token '{}'", &capture[0]);
                };
            tokens.push(token)
        }
        
        // Filter whitespace
        tokens.into_iter().filter(|token|
            if let Token::Whitespace = *token {
                false
            } else {
                true
            }).collect()
    }

    /// Get the regular expression that can capture all Tokens. The regular
    /// expression can capture Unicode characters.
    ///
    /// # Return
    /// An unwrapped Regex object.
    pub fn regex() -> regex::Regex {
        regex::Regex::from_str(concat!(

            // Reserved keywords
            r"(?P<As>as)|",
            r"(?P<Else>else)|",
            r"(?P<Expose>expose)|",
            r"(?P<Extern>extern)|",
            r"(?P<For>for)|",
            r"(?P<Func>fn)|",
            r"(?P<If>if)|",
            r"(?P<Import>import)|",
            r"(?P<In>in)|",
            r"(?P<Module>module)|",
            r"(?P<Mut>mut)|",
            r"(?P<Ref>ref)|",
            r"(?P<Type>type)|",

            // Symbols
            r"(?P<BraceL>\{)|",
            r"(?P<BraceR>\})|",
            r"(?P<BracketL>\[)|",
            r"(?P<BracketR>\])|",
            r"(?P<Colon>:)|",
            r"(?P<Comma>,)|",
            r"(?P<Dot>\.)|",
            r"(?P<DotDot>\.\.)|",
            r"(?P<LambdaR>->)|",
            r"(?P<ParenL>\()|",
            r"(?P<ParenR>\))|",
            r"(?P<SemiColon>;)|",
            r"(?P<Whitespace>( |\t|\n|\r)+)|",

            // Operators
            r"(?P<Add>\+)|",
            r"(?P<AddEq>\+=)|",
            r"(?P<And>&&)|",
            r"(?P<Assign>:=)|",
            r"(?P<Div>/)|",
            r"(?P<DivEq>/=)|",
            r"(?P<Equal>=)|",
            r"(?P<LessThan><)|",
            r"(?P<LessThanEq><=)|",
            r"(?P<GreaterThan>>)|",
            r"(?P<GreaterThanEq>>=)|",
            r"(?P<Mul>\*)|",
            r"(?P<MulEq>\*=)|",
            r"(?P<Or>\|\|)|",
            r"(?P<PushPop><-)|",
            r"(?P<Sub>-)|",
            r"(?P<SubEq>-=)|",

            // Literals
            r"'(?P<Char>(\\'|[^']|\\u([0-9]|[ABCDEF]){4}))'|",
            r"(?P<Bool>(true|false))|",
            r"(?P<Float>[0-9]+\.[0-9]+)|",
            r"(?P<Int>[0-9]+)|",
            r#""(?P<Str>(\\"|[^"])*)"|"#,

            // Idents
            r"(?P<Ident>[_a-zA-Z]([_a-zA-Z]|[0-9])*)|",

            // Coments
            r"(?P<Comment>//(.)*\n)|",

            // Unexpected
            r"(.)",
        )).unwrap()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        match *self {

            // Reserved keywords
            As => write!(formatter, "as"),
            Else => write!(formatter, "else"),
            Extern => write!(formatter, "extern"),
            Expose => write!(formatter, "expose"),
            For => write!(formatter, "for"),
            Func => write!(formatter, "fn"),
            If => write!(formatter, "if"),
            Import => write!(formatter, "import"),
            In => write!(formatter, "in"),
            Module => write!(formatter, "module"),
            Mut => write!(formatter, "mut"),
            Ref => write!(formatter, "ref"),
            Type => write!(formatter, "type"),

            // Symbols
            BraceL => write!(formatter, "{{"),
            BraceR => write!(formatter, "}}"),
            BracketL => write!(formatter, "["),
            BracketR => write!(formatter, "]"),
            Colon => write!(formatter, ":"),
            Comma => write!(formatter, ","),
            Dot => write!(formatter, "."),
            DotDot => write!(formatter, ".."),
            LambdaR => write!(formatter, "->"),
            ParenL => write!(formatter, "("),
            ParenR => write!(formatter, ")"),
            SemiColon => write!(formatter, ";"),
            Whitespace => write!(formatter, " "),

            // Operators
            Add => write!(formatter, "+"),
            AddEq => write!(formatter, "+="),
            And => write!(formatter, "&&"),
            Assign => write!(formatter, ":="),
            Div => write!(formatter, "/"),
            DivEq => write!(formatter, "/="),
            Equal => write!(formatter, "="),
            GreaterThan => write!(formatter, ">"),
            GreaterThanEq => write!(formatter, ">="),
            LessThan => write!(formatter, "<"),
            LessThanEq => write!(formatter, "<="),
            Mul => write!(formatter, "*"),
            MulEq => write!(formatter, "*="),
            Or => write!(formatter, "||"),
            PushPop => write!(formatter, "<-"),
            Sub => write!(formatter, "-"),
            SubEq => write!(formatter, "-="),

            // Literals
            Char(ref character) => write!(formatter, "{}", character),
            Bool(ref boolean) => write!(formatter, "{}", boolean),
            Float(ref float) => write!(formatter, "{}", float),
            Int(ref int) => write!(formatter, "{}", int),
            Str(ref string) => write!(formatter, "{}", string),

            // Idents
            Ident(ref ident) => write!(formatter, "{}", ident),

            // Comments
            Comment(ref comment) => write!(formatter, "//{}", comment),
        }
    }
}