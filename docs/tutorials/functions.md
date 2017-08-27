# Functions

## Declaring a function

When declaring a function, it is necessary to declare its type profile. The type profile includes the function name, arguments, and return type. The name of a function does not need to be unique, as long as the type profile can be distinguished from other functions of the same name.

```arvo
fn writeln(arg string) void
fn writeln(arg i64) void
```

## Defining a function

A function can optionally have a body, defining the behaviour of the function when it is called. If the body of the function is absent, it must be present in an external library that can be linked. A function body can be any expression that returns a value.

```arvo
fn add(x i64, y i64) i64 -> x + y
fn sub(x i64, y i64) i64 -> {
    x - y
}
fn div(x i64, y i64) i64? -> if y = 0 {
    nil
} else {
    x / y
}
```

## Passing by reference

You can use the `ref` keyword to declare a function that accepts arguments by reference, instead of accepting them by value (which would move, or copy, the original value).

```arvo
fn passByReference(arg ref string) void
```

To prevent other functions from referencing a value, you can use the `mut ref` keywords to create a "**mut**able", or "**mut**ually exclusive", reference. Other references cannot exist alongide a `mut ref` and so they can be used to provide locking mechanisms around resources.

```arvo
fn (+=)(mut ref x i64, y i64) void -> {
    let tmp i64 := deref x + y
    mut deref x = tmp
}

fn increment(x mut ref i64, y i64) void -> x += y

fn foo() i64 -> {
    let mut x i64 := 1
    increment(mut ref x, 2)
    increment(mut ref x, 3)
}

fn main() void -> writeln(foo())
```

The output from the above function will be `6`. If the `mut ref` keywords did not lock values, then the possible outputs could include `3`, and `4`, which is definitely not what this program looks like it should output. For more information about this phenomenon do some Googling around data racing in concurrent programming.

## Foreign functions

You can declare functions that are defined in another language using the `extern` keyword. Such functions must only include references and primitive types (excluding channels) in their type profile. Variadic arguments are not supported.

```arvo
extern fn declaredInArvoDefinedInC(x i64, y i64) i64
extern fn declaredInCDefinedInArvo(x i64, y i64) i64 -> x + y
```