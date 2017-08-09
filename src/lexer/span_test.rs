use super::span::{Position, Span};

#[test]
fn test_position() {
    assert_eq!(
        Position::new("test.arvo", 13, 42),
        Position::new("test.arvo", 13, 42)
    );
    assert!(
        Position::new("test.arvo", 13, 42) !=
        Position::new("test.arvo", 26, 84)
    );
    assert!(
        Position::new("test1.arvo", 13, 42) !=
        Position::new("test2.arvo", 13, 42)
    );
}

#[test]
fn test_span() {
    assert_eq!(
        Span::new("test.arvo", 13, 42, 26, 84),
        Span::new("test.arvo", 13, 42, 26, 84)
    );
    assert_eq!(
        Span::new("test.arvo", 13, 42, 26, 84).begin(),
        Span::new("test.arvo", 13, 42, 52, 168).begin()
    );
    assert_eq!(
        Span::new("test.arvo", 13, 42, 26, 84).end(),
        Span::new("test.arvo", 52, 168, 26, 84).end()
    );
    assert!(
        Span::new("test.arvo", 13, 42, 26, 84) !=
        Span::new("test.arvo", 26, 84, 26, 84)
    );
    assert!(
        Span::new("test.arvo", 13, 42, 26, 84) !=
        Span::new("test.arvo", 13, 42, 52, 168)
    );
    assert!(
        Span::new("test1.arvo", 13, 42, 26, 84) !=
        Span::new("test2.arvo", 13, 42, 26, 84)
    );
    assert!(
        Span::new("test.arvo", 13, 42, 26, 84).begin() !=
        Span::new("test.arvo", 26, 84, 52, 168).begin()
    );
    assert!(
        Span::new("test.arvo", 13, 42, 26, 84).end() !=
        Span::new("test.arvo", 26, 84, 52, 168).end()
    );
}