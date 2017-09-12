//! The Token module defines the different lexical tokens in the Arvo
//! programming language, and defines some basic operations on these tokens.

// External crate dependencies
use regex;

use std::char;
use std::fmt;
use std::str::FromStr;

use super::span::{Span, Spanned};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Precedence(u8);

impl Precedence {

    pub fn new(prec: u8) -> Precedence {
        Precedence(prec)
    }

    pub fn lowest() -> Precedence {
        Precedence(0)
    }

    pub fn highest() -> Precedence {
        Precedence(255)
    }

    pub fn next(&self) -> Precedence {
        Precedence::new(self.0 + 1)
    }
}

/// A Token represents a lexical token in the Arvo programming language. It
/// includes the values of literals, identifiers, and comments.
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Reserved keywords
    As(Span),
    Else(Span),
    Extern(Span),
    Expose(Span),
    For(Span),
    Func(Span),
    If(Span),
    Import(Span),
    In(Span),
    Module(Span),
    Mut(Span),
    Ref(Span),
    Type(Span),

    // Operators
    Add(Span),
    AddEq(Span),
    And(Span),
    Assign(Span),
    Div(Span),
    DivEq(Span),
    Equal(Span),
    NotEqual(Span),
    GreaterThan(Span),
    GreaterThanEq(Span),
    LessThan(Span),
    LessThanEq(Span),
    Mul(Span),
    MulEq(Span),
    Or(Span),
    PushPop(Span),
    Sub(Span),
    SubEq(Span),

    // Symbols
    BraceL(Span),
    BraceR(Span),
    BracketL(Span),
    BracketR(Span),
    Colon(Span),
    Comma(Span),
    Dot(Span),
    DotDot(Span),
    LambdaR(Span),
    ParenL(Span),
    ParenR(Span),
    Pipe(Span),
    Question(Span),
    SemiColon(Span),
    Whitespace(Span),
    WhitespaceNewline(Span),

    // Literals
    Char(char, Span),
    Bool(bool, Span),
    Float(f64, Span),
    Int(i64, Span),
    Str(String, Span),

    // Idents
    Ident(String, Span),

    // Comments
    Comment(String, Span),
}

impl Token {
    pub fn tokenise(filename: &str, input: &str) -> Vec<Token> {
        let mut line = 1;
        let mut column = 1;
        let mut tokens = Vec::new();

        for capture in Token::regex().captures_iter(input) {

            let begin_line = line;
            let begin_column = column;
            column += (&capture[0]).to_string().chars().count();
            let span = Span::new(filename, begin_line, begin_column, line, column - 1);
            if &capture[0] == "\n" {
                line += 1;
                column = 1;
            }

            let token = 

                // Reserved keywords
                if capture.name("As").is_some() {
                    Token::As(span)
                } else if capture.name("Else").is_some() {
                    Token::Else(span)
                } else if capture.name("Expose").is_some() {
                    Token::Expose(span)
                } else if capture.name("Extern").is_some() {
                    Token::Extern(span)
                } else if capture.name("For").is_some() {
                    Token::For(span)
                } else if capture.name("Func").is_some() {
                    Token::Func(span)
                } else if capture.name("If").is_some() {
                    Token::If(span)
                } else if capture.name("Import").is_some() {
                    Token::Import(span)
                } else if capture.name("In").is_some() {
                    Token::In(span)
                } else if capture.name("Module").is_some() {
                    Token::Module(span)
                } else if capture.name("Mut").is_some() {
                    Token::Mut(span)
                } else if capture.name("Ref").is_some() {
                    Token::Ref(span)
                } else if capture.name("Type").is_some() {
                    Token::Type(span)
                }

                // Symbols
                else if capture.name("BraceL").is_some() {
                    Token::BraceL(span)
                } else if capture.name("BraceR").is_some() {
                    Token::BraceR(span)
                } else if capture.name("BracketL").is_some() {
                    Token::BracketL(span)
                } else if capture.name("BracketR").is_some() {
                    Token::BracketR(span)
                } else if capture.name("Colon").is_some() {
                    Token::Colon(span)
                } else if capture.name("Comma").is_some() {
                    Token::Comma(span)
                } else if capture.name("DotDot").is_some() {
                    Token::DotDot(span)
                } else if capture.name("Dot").is_some() {
                    Token::Dot(span)
                } else if capture.name("LambdaR").is_some() {
                    Token::LambdaR(span)
                } else if capture.name("ParenL").is_some() {
                    Token::ParenL(span)
                } else if capture.name("ParenR").is_some() {
                    Token::ParenR(span)
                } else if capture.name("Pipe").is_some() {
                    Token::Pipe(span)
                } else if capture.name("Question").is_some() {
                    Token::Question(span)
                } else if capture.name("SemiColon").is_some() {
                    Token::SemiColon(span)
                } else if capture.name("Whitespace").is_some() {
                    Token::Whitespace(span)
                } else if capture.name("WhitespaceNewline").is_some() {
                    Token::WhitespaceNewline(span)
                }
                
                // Operators
                else if capture.name("Add").is_some() {
                    Token::Add(span)
                } else if capture.name("AddEq").is_some() {
                    Token::AddEq(span)
                } else if capture.name("And").is_some() {
                    Token::And(span)
                } else if capture.name("Assign").is_some() {
                    Token::Assign(span)
                } else if capture.name("Div").is_some() {
                    Token::Div(span)
                } else if capture.name("DivEq").is_some() {
                    Token::DivEq(span)
                } else if capture.name("Equal").is_some() {
                    Token::Equal(span)
                } else if capture.name("GreaterThan").is_some() {
                    Token::GreaterThan(span)
                } else if capture.name("GreaterThanEq").is_some() {
                    Token::GreaterThanEq(span)
                } else if capture.name("LessThan").is_some() {
                    Token::LessThan(span)
                } else if capture.name("LessThanEq").is_some() {
                    Token::LessThanEq(span)
                } else if capture.name("Mul").is_some() {
                    Token::Mul(span)
                } else if capture.name("MulEq").is_some() {
                    Token::MulEq(span)
                } else if capture.name("Or").is_some() {
                    Token::Or(span)
                } else if capture.name("PushPop").is_some() {
                    Token::PushPop(span)
                } else if capture.name("Sub").is_some() {
                    Token::Sub(span)
                } else if capture.name("SubEq").is_some() {
                    Token::SubEq(span)
                }

                // Literals
                else if capture.name("Bool").is_some() {
                    Token::Bool(bool::from_str(capture.name("Bool").unwrap()).unwrap(), span)
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
                        }, span)
                } else if capture.name("Float").is_some() {
                    Token::Float(f64::from_str(capture.name("Float").unwrap()).unwrap(), span)
                } else if capture.name("Int").is_some() {
                    Token::Int(i64::from_str(capture.name("Int").unwrap()).unwrap(), span)
                } else if capture.name("Str").is_some() {
                    Token::Str(capture.name("Str").unwrap().to_string(), span)
                }

                // Idents
                else if capture.name("Ident").is_some() {
                    Token::Ident(capture.name("Ident").unwrap().to_string(), span)
                }

                // Comments
                else if capture.name("Comment").is_some() {
                    Token::Comment(capture.name("Comment").unwrap().to_string(), span)
                }
                
                // Other
                else {
                    panic!("unexpected token '{}'", &capture[0]);
                };
            tokens.push(token)
        }
        
        // Filter whitespace
        tokens.into_iter().filter(|token| match *token {
            Token::Whitespace(..) => false,
            _ => true,
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
            r"(?P<DotDot>\.\.)|",
            r"(?P<Dot>\.)|",
            r"(?P<LambdaR>->)|",
            r"(?P<ParenL>\()|",
            r"(?P<ParenR>\))|",
            r"(?P<Pipe>\|)|",
            r"(?P<Question>\?)|",
            r"(?P<SemiColon>;)|",
            r"(?P<Whitespace>( |\t))|",
            r"(?P<WhitespaceNewline>(\n|\r))|",

            // Operators
            r"(?P<Add>\+)|",
            r"(?P<AddEq>\+=)|",
            r"(?P<And>&&)|",
            r"(?P<Assign>:=)|",
            r"(?P<Div>/)|",
            r"(?P<DivEq>/=)|",
            r"(?P<Equal>=)|",
            r"(?P<NotEqual>!=)|",
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

    pub fn precedence(&self) -> Precedence {
        use self::Token::*;

        match *self {
            Or(..) => Precedence(1),
            And(..) => Precedence(2),
            Equal(..) |
            NotEqual(..) |
            LessThan(..) |
            LessThanEq(..) |
            GreaterThan(..) |
            GreaterThanEq(..) => Precedence(3),
            Add(..) |
            Sub(..) => Precedence(4),
            Mul(..) |
            Div(..) => Precedence(5),
            _ => Precedence::lowest(),
        }
    }

    pub fn is_whitespace(&self) -> bool {
        match *self {
            Token::Whitespace(..) | Token::WhitespaceNewline(..) => true,
            _ => false,
        }
    }
}

impl Spanned for Token {
     fn span(&self) -> &Span {
        use self::Token::*;
        
        match *self {

            // Reserved keywords
            As(ref span, ..) => span,
            Else(ref span, ..) => span,
            Extern(ref span, ..) => span,
            Expose(ref span, ..) => span,
            For(ref span, ..) => span,
            Func(ref span, ..) => span,
            If(ref span, ..) => span,
            Import(ref span, ..) => span,
            In(ref span, ..) => span,
            Module(ref span, ..) => span,
            Mut(ref span, ..) => span,
            Ref(ref span, ..) => span,
            Type(ref span, ..) => span,

            // Symbols
            BraceL(ref span, ..) => span,
            BraceR(ref span, ..) => span,
            BracketL(ref span, ..) => span,
            BracketR(ref span, ..) => span,
            Colon(ref span, ..) => span,
            Comma(ref span, ..) => span,
            Dot(ref span, ..) => span,
            DotDot(ref span, ..) => span,
            LambdaR(ref span, ..) => span,
            ParenL(ref span, ..) => span,
            ParenR(ref span, ..) => span,
            Pipe(ref span, ..) => span,
            Question(ref span, ..) => span,
            SemiColon(ref span, ..) => span,
            Whitespace(ref span, ..) => span,
            WhitespaceNewline(ref span, ..) => span,

            // Operators
            Add(ref span, ..) => span,
            AddEq(ref span, ..) => span,
            And(ref span, ..) => span,
            Assign(ref span, ..) => span,
            Div(ref span, ..) => span,
            DivEq(ref span, ..) => span,
            Equal(ref span, ..) => span,
            NotEqual(ref span, ..) => span,
            GreaterThan(ref span, ..) => span,
            GreaterThanEq(ref span, ..) => span,
            LessThan(ref span, ..) => span,
            LessThanEq(ref span, ..) => span,
            Mul(ref span, ..) => span,
            MulEq(ref span, ..) => span,
            Or(ref span, ..) => span,
            PushPop(ref span, ..) => span,
            Sub(ref span, ..) => span,
            SubEq(ref span, ..) => span,

            // Literals
            Char(_, ref span, ..) => span,
            Bool(_, ref span, ..) => span,
            Float(_, ref span, ..) => span,
            Int(_, ref span, ..) => span,
            Str(_, ref span, ..) => span,

            // Idents
            Ident(_, ref span, ..) => span,

            // Comments
            Comment(_, ref span, ..) => span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        use self::Token::*;
        
        match *self {

            // Reserved keywords
            As(ref mut span, ..) => span,
            Else(ref mut span, ..) => span,
            Extern(ref mut span, ..) => span,
            Expose(ref mut span, ..) => span,
            For(ref mut span, ..) => span,
            Func(ref mut span, ..) => span,
            If(ref mut span, ..) => span,
            Import(ref mut span, ..) => span,
            In(ref mut span, ..) => span,
            Module(ref mut span, ..) => span,
            Mut(ref mut span, ..) => span,
            Ref(ref mut span, ..) => span,
            Type(ref mut span, ..) => span,

            // Symbols
            BraceL(ref mut span, ..) => span,
            BraceR(ref mut span, ..) => span,
            BracketL(ref mut span, ..) => span,
            BracketR(ref mut span, ..) => span,
            Colon(ref mut span, ..) => span,
            Comma(ref mut span, ..) => span,
            Dot(ref mut span, ..) => span,
            DotDot(ref mut span, ..) => span,
            LambdaR(ref mut span, ..) => span,
            ParenL(ref mut span, ..) => span,
            ParenR(ref mut span, ..) => span,
            Pipe(ref mut span, ..) => span,
            Question(ref mut span, ..) => span,
            SemiColon(ref mut span, ..) => span,
            Whitespace(ref mut span, ..) => span,
            WhitespaceNewline(ref mut span, ..) => span,

            // Operators
            Add(ref mut span, ..) => span,
            AddEq(ref mut span, ..) => span,
            And(ref mut span, ..) => span,
            Assign(ref mut span, ..) => span,
            Div(ref mut span, ..) => span,
            DivEq(ref mut span, ..) => span,
            NotEqual(ref mut span, ..) => span,
            Equal(ref mut span, ..) => span,
            GreaterThan(ref mut span, ..) => span,
            GreaterThanEq(ref mut span, ..) => span,
            LessThan(ref mut span, ..) => span,
            LessThanEq(ref mut span, ..) => span,
            Mul(ref mut span, ..) => span,
            MulEq(ref mut span, ..) => span,
            Or(ref mut span, ..) => span,
            PushPop(ref mut span, ..) => span,
            Sub(ref mut span, ..) => span,
            SubEq(ref mut span, ..) => span,

            // Literals
            Char(_, ref mut span, ..) => span,
            Bool(_, ref mut span, ..) => span,
            Float(_, ref mut span, ..) => span,
            Int(_, ref mut span, ..) => span,
            Str(_, ref mut span, ..) => span,

            // Idents
            Ident(_, ref mut span, ..) => span,

            // Comments
            Comment(_, ref mut span, ..) => span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;

        match *self {

            // Reserved keywords
            As(..) => write!(formatter, "as"),
            Else(..) => write!(formatter, "else"),
            Extern(..) => write!(formatter, "extern"),
            Expose(..) => write!(formatter, "expose"),
            For(..) => write!(formatter, "for"),
            Func(..) => write!(formatter, "fn"),
            If(..) => write!(formatter, "if"),
            Import(..) => write!(formatter, "import"),
            In(..) => write!(formatter, "in"),
            Module(..) => write!(formatter, "module"),
            Mut(..) => write!(formatter, "mut"),
            Ref(..) => write!(formatter, "ref"),
            Type(..) => write!(formatter, "type"),

            // Symbols
            BraceL(..) => write!(formatter, "{{"),
            BraceR(..) => write!(formatter, "}}"),
            BracketL(..) => write!(formatter, "["),
            BracketR(..) => write!(formatter, "]"),
            Colon(..) => write!(formatter, ":"),
            Comma(..) => write!(formatter, ","),
            Dot(..) => write!(formatter, "."),
            DotDot(..) => write!(formatter, ".."),
            LambdaR(..) => write!(formatter, "->"),
            ParenL(..) => write!(formatter, "("),
            ParenR(..) => write!(formatter, ")"),
            Pipe(..) => write!(formatter, "|"),
            Question(..) => write!(formatter, "?"),
            SemiColon(..) => write!(formatter, ";"),
            Whitespace(..) => write!(formatter, " "),
            WhitespaceNewline(..) => write!(formatter, "\n"),

            // Operators
            Add(..) => write!(formatter, "+"),
            AddEq(..) => write!(formatter, "+="),
            And(..) => write!(formatter, "&&"),
            Assign(..) => write!(formatter, ":="),
            Div(..) => write!(formatter, "/"),
            DivEq(..) => write!(formatter, "/="),
            NotEqual(..) => write!(formatter, "!="),
            Equal(..) => write!(formatter, "="),
            GreaterThan(..) => write!(formatter, ">"),
            GreaterThanEq(..) => write!(formatter, ">="),
            LessThan(..) => write!(formatter, "<"),
            LessThanEq(..) => write!(formatter, "<="),
            Mul(..) => write!(formatter, "*"),
            MulEq(..) => write!(formatter, "*="),
            Or(..) => write!(formatter, "||"),
            PushPop(..) => write!(formatter, "<-"),
            Sub(..) => write!(formatter, "-"),
            SubEq(..) => write!(formatter, "-="),

            // Literals
            Char(ref character, ..) => write!(formatter, "{}", character),
            Bool(ref boolean, ..) => write!(formatter, "{}", boolean),
            Float(ref float, ..) => write!(formatter, "{}", float),
            Int(ref int, ..) => write!(formatter, "{}", int),
            Str(ref string, ..) => write!(formatter, "{}", string),

            // Idents
            Ident(ref ident, ..) => write!(formatter, "{}", ident),

            // Comments
            Comment(ref comment, ..) => write!(formatter, "//{}", comment),
        }
    }
}

pub type Tokens = Vec<Token>;