use super::*;

#[test]
fn test_main() {
    let source = r#"
        fn main() -> {}
    "#;
}

#[test]
fn test_hello_world() {
    let source = r#"
        fn main() -> {
            writeln("arvo");
            writeln("pärt");
        }
    "#;
}

#[test]
fn test_if() {
    let source = r#"
        fn main() -> {
            if readln() = "arvo" {
                writeln("you said arvo");
            } else {
                writeln("you did not say arvo");
            }
        }
    "#;
}

#[test]
fn test_for() {
    let source = r#"
        fn main() -> {
            for i in 1 .. 100 {
                writeln(i);
            }
        }
    "#;
}

#[test]
fn test_producer_consumer() {
    let source = r#"
        fn main() -> {
            let m := 3;
            let n := 4;
            let xs := .. ();
            for _ in 1 .. m {
                producer(xs);
            };
            for _ in 1 .. n {
                consumer(xs);
            };
        }

        fn producer(mut xs ..i64) -> 
            for x in 1 .. 100 {
                xs <- x;
            }

        fn consumer(xs ..i64) -> 
            for x in xs {
                writeln(xs);
            }
    "#;
}

#[test]
fn test_increment() {
    let source = r#"
        fn main() -> {
            let mut x := 1;
            x += 2;
            x += 3;
        }
    "#;
}