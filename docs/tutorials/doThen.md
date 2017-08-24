# Do Then

A `do then` expression can be used to order two expressions. The `do`
expression will always end before the `then` expression begins, however within
each expression normal ordering rules apply.

```
fn main() -> {

  do writeln("before") 
  then writeln("after")

  do {
    writeln("first or second")
    writeln("second or first")
  } then {
    writeln("third or fourth")
    writeln("fourth or third")
  }
}
```