# Modules

## Defining a module

Use the `module` keyword to declare the module for a file. All Arvo files must declare a module and files that are in the same folder must declare the same module (the module name does not have to match the parent folder).

```arvo
module main

fn main () -> {
    writeln("hello, world!");
}
```

## Importing a module

A module must be imported before it can be used. To import a module, use the `import` keyword and name a folder. The module files within that folder will be imported, and will be available through the declared module name.

Imports are resolved in the following order:
1. Relative to the importing file
2. Relative to any directory defined as an argument to the compiler
3. Relative to any directory defined by the $ARVO_USE environment variable
4. Relative to any directory defined by the standard library

```arvo
import io

fn main() -> {
    let mut f = io.open("example.txt");
    f.write("Hello, world!");
}
```

## Renaming modules on import

The `as` keyword can be used to explicitly name a module when it is imported. By default, the module will be accessible by its declared name.

```
import io as inputOutput

fn main() -> {
    let mut f = inputOutput.open("example.txt");
    f.write("Hello, world!");
}
```

