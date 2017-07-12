//! The Token module defines the different lexical tokens in the Arvo
//! programming language, and defines some basic operations on these tokens.

// External crate dependencies
use regex::Regex;

// Standard library dependencies
use std::fmt;

// Test module
#[cfg(test)]
mod test;

/// A Token represents a lexical token in the Arvo programming language. It
/// includes the values of literals, identifiers, and comments.
#[derive(Clone)]
#[derive(Debug)]
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

    // Whitespace
    CarriageReturn,
    Newline,
    Space,
    Tab,

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

    /// Get the regular expression that can capture the Token. The regular
    /// expressions returned support Unicode characters.
    ///
    /// # Return
    /// An unwrapped Regex object.
    pub fn regex(&self) -> Regex {
        use self::Token::*;

        match *self {

            // Reserved keywords
            As => Regex::new(r"as").unwrap(),
            Else => Regex::new(r"else").unwrap(),
            Extern => Regex::new(r"extern").unwrap(),
            Expose => Regex::new(r"expose").unwrap(),
            For => Regex::new(r"for").unwrap(),
            Func => Regex::new(r"fn").unwrap(),
            If => Regex::new(r"if").unwrap(),
            Import => Regex::new(r"import").unwrap(),
            In => Regex::new(r"in").unwrap(),
            Module => Regex::new(r"module").unwrap(),
            Mut => Regex::new(r"mut").unwrap(),
            Ref => Regex::new(r"ref").unwrap(),
            Type => Regex::new(r"type").unwrap(),

            // Operators
            Add => Regex::new(r"\+").unwrap(),
            AddEq => Regex::new(r"\+=").unwrap(),
            And => Regex::new(r"&&").unwrap(),
            Assign => Regex::new(r":=").unwrap(),
            Div => Regex::new(r"/").unwrap(),
            DivEq => Regex::new(r"/=").unwrap(),
            Equal => Regex::new(r"=").unwrap(),
            LessThan => Regex::new(r"<").unwrap(),
            LessThanEq => Regex::new(r"<=").unwrap(),
            GreaterThan => Regex::new(r">").unwrap(),
            GreaterThanEq => Regex::new(r">=").unwrap(),
            Mul => Regex::new(r"\*").unwrap(),
            MulEq => Regex::new(r"\*=").unwrap(),
            Or => Regex::new(r"\|\|").unwrap(),
            PushPop => Regex::new(r"<-").unwrap(),
            Sub => Regex::new(r"-").unwrap(),
            SubEq => Regex::new(r"-=").unwrap(),

            // Symbols
            BraceL => Regex::new(r"\{").unwrap(),
            BraceR => Regex::new(r"\}").unwrap(),
            BracketL => Regex::new(r"\[").unwrap(),
            BracketR => Regex::new(r"\]").unwrap(),
            Colon => Regex::new(r":").unwrap(),
            Comma => Regex::new(r",").unwrap(),
            Dot => Regex::new(r"\.").unwrap(),
            DotDot => Regex::new(r"\.\.").unwrap(),
            LambdaR => Regex::new(r"->").unwrap(),
            ParenL => Regex::new(r"\(").unwrap(),
            ParenR => Regex::new(r"\)").unwrap(),
            SemiColon => Regex::new(r";").unwrap(),

            // Whitespace
            CarriageReturn => Regex::new(r"\r").unwrap(),
            Newline => Regex::new(r"\n").unwrap(),
            Space => Regex::new(r" ").unwrap(),
            Tab => Regex::new(r"\t").unwrap(),

            // Literals
            Char(..) => Regex::new(r"'(\\'|[^']|\\u([0-9]|[ABCDEF]){4})?'").unwrap(),
            Bool(..) => Regex::new(r"(true|false)").unwrap(),
            Float(..) => Regex::new(r"[0-9]+\.[0-9]+").unwrap(),
            Int(..) => Regex::new(r"[0-9]+").unwrap(),
            Str(..) => Regex::new(r#""(\\"|[^"])*""#).unwrap(),

            // Idents
            Ident(..) => Regex::new(r"[_a-zA-Z]([_a-zA-Z]|[0-9])*").unwrap(),

            // Coments
            Comment(..) => Regex::new(r"//(.)*\n").unwrap(),
        }
    }
}

/// A Position represents an arbitrary source position. It includes the
/// filename, line number, and column number.
#[derive(Clone)]
#[derive(Debug)]
pub struct Position {
    filename: String,
    line: u64,
    column: u64,
}

impl Position {

    /// Create a new Position.
    ///
    /// # Arguments
    /// * `filename` The name of the file.
    /// * `line` The line number, where the first line of the file is at zero.
    /// * `column` The column number, where the first column of a line is at zero.
    ///
    /// # Return
    /// A new Position.
    pub fn new<Filename>(filename: Filename, line: u64, column: u64) -> Position where Filename: ToString {
        Position {
            filename: filename.to_string(),
            line: line,
            column: column,
        }
    }

    /// # Return
    /// The name of the file in which the Position exists.
    pub fn filename(&self) -> &String {
        &self.filename
    }

    /// # Return
    /// The line number of the Position.
    pub fn line(&self) -> u64 {
        self.line
    }

    /// # Return
    /// The column number of the Position.
    pub fn column(&self) -> u64 {
        self.column
    }
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.filename.len() > 0 {
            write!(formatter, "{}:{}:{}", self.filename, self.line, self.column)
        } else {
            write!(formatter, "{}:{}", self.line, self.column)
        }
    }
}

/// A Span represents an arbitrary source range. It includes the beginning and
/// ending Positions.
#[derive(Clone)]
#[derive(Debug)]
pub struct Span {
    begin: Position,
    end: Position,
}

impl Span {

    /// Create a new Span. The beginning and ending Positions will also be
    /// created. The beginning and ending Positions are guaranteed to have the
    /// same filename.
    ///
    /// # Arguments
    /// * `filename` The name of the file. This name will be used for the
    ///   beginning and ending Positions.
    /// * `begin_line` The line number for the beginning of the Span, where
    ///   the first line of the file is at zero.
    /// * `begin_column` The column number for the beginning of the Span, 
    ///   where the first column of a line is at zero.
    /// * `end_line` The line number for the ending of the Span, where
    ///   the first line of the file is at zero.
    /// * `end_column` The column number for the ending of the Span, 
    ///   where the first column of a line is at zero.
    ///
    /// # Return
    /// A new Span, with new beginning and ending Positions.
    pub fn new<Filename>(filename: Filename, begin_line: u64, begin_column: u64, end_line: u64, end_column: u64) -> Span where Filename: Clone + ToString {
        Span {
            begin: Position::new(filename.clone(), begin_line, begin_column),
            end: Position::new(filename.clone(), end_line, end_column)
        }
    }

    /// # Return
    /// The beginning Position of the Span.
    pub fn begin(&self) -> &Position {
        &self.begin
    }

    /// # Return
    /// The ending Position of the Span.
    pub fn end(&self) -> &Position {
        &self.end
    }
}