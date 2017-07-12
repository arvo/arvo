#[test]
fn test_regex() {
    use super::Token::*;

    // Reserved keywords
    assert!(As.regex().is_match("as"));
    assert!(Else.regex().is_match("else"));
    assert!(Extern.regex().is_match("extern"));
    assert!(Expose.regex().is_match("expose"));
    assert!(For.regex().is_match("for"));
    assert!(Func.regex().is_match("fn"));
    assert!(If.regex().is_match("if"));
    assert!(Import.regex().is_match("import"));
    assert!(In.regex().is_match("in"));
    assert!(Module.regex().is_match("module"));
    assert!(Mut.regex().is_match("mut"));
    assert!(Ref.regex().is_match("ref"));
    assert!(Type.regex().is_match("type"));

    // Operators
    assert!(Add.regex().is_match("+"));
    assert!(AddEq.regex().is_match("+="));
    assert!(And.regex().is_match("&&"));
    assert!(Assign.regex().is_match(":="));
    assert!(Div.regex().is_match("/"));
    assert!(DivEq.regex().is_match("/="));
    assert!(Equal.regex().is_match("="));
    assert!(GreaterThan.regex().is_match(">"));
    assert!(GreaterThanEq.regex().is_match(">="));
    assert!(LessThan.regex().is_match("<"));
    assert!(LessThanEq.regex().is_match("<="));
    assert!(Mul.regex().is_match("*"));
    assert!(MulEq.regex().is_match("*="));
    assert!(Or.regex().is_match("||"));
    assert!(PushPop.regex().is_match("<-"));
    assert!(Sub.regex().is_match("-"));
    assert!(SubEq.regex().is_match("-="));

    // Symbols
    assert!(BraceL.regex().is_match("{"));
    assert!(BraceR.regex().is_match("}"));
    assert!(BracketL.regex().is_match("["));
    assert!(BracketR.regex().is_match("]"));
    assert!(Colon.regex().is_match(":"));
    assert!(Comma.regex().is_match(","));
    assert!(Dot.regex().is_match("."));
    assert!(DotDot.regex().is_match(".."));
    assert!(LambdaR.regex().is_match("->"));
    assert!(ParenL.regex().is_match("("));
    assert!(ParenR.regex().is_match(")"));
    assert!(SemiColon.regex().is_match(";"));

    // Whitespace
    assert!(CarriageReturn.regex().is_match("\r"));
    assert!(Newline.regex().is_match("\n"));
    assert!(Space.regex().is_match(" "));
    assert!(Tab.regex().is_match("\t"));

    // Literals
    assert!(Char('a').regex().is_match("'a'"));
    assert!(Char('ä').regex().is_match("'ä'"));
    assert!(Char('\u{00E4}').regex().is_match("'\\u00E4'"));
    assert!(Bool(true).regex().is_match("true"));
    assert!(Bool(false).regex().is_match("false"));
    assert!(Float(0.0).regex().is_match("0.0"));
    assert!(Float(3.14).regex().is_match("3.14"));
    assert!(Float(1234.5678).regex().is_match("1234.5678"));
    assert!(Int(0).regex().is_match("0"));
    assert!(Int(314).regex().is_match("314"));
    assert!(Int(12345678).regex().is_match("12345678"));
    assert!(Str("Arvo Part".to_string()).regex().is_match("\"Arvo Part\""));
    assert!(Str("Arvo Pärt".to_string()).regex().is_match("\"Arvo Pärt\""));
    assert!(Str("Arvo Pärt ❤".to_string()).regex().is_match("\"Arvo Pärt ❤\""));

    // Idents
    assert!(Ident("arvopart".to_string()).regex().is_match("arvopart"));
    assert!(Ident("arvo_part".to_string()).regex().is_match("arvo_part"));
    assert!(Ident("arvoPart".to_string()).regex().is_match("arvoPart"));
    assert!(Ident("Arvo_Part".to_string()).regex().is_match("Arvo_Part"));
    assert!(Ident("ArvoPart".to_string()).regex().is_match("ArvoPart"));

    // Comments
    assert!(Comment("This is a commment".to_string()).regex().is_match("// This is a commment\n"));
}