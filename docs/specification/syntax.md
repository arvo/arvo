# Syntax

All source files must be UTF-8 encoded. The grammar is given in backus-naur form unless stated otherwise.

## Module declarations

All source files must begin with a module declaration.

```
module_decl ::= "module" identifier module_statements_opt
module_statements_opt ::= module_statements | ""
module_statements ::= module_statements module_statement
                    | module_statement
module_statement ::= function_decl
                   | type_decl
```

## Function declarations

```
function_decl ::= expose_opt extern_opt "fn" identifier "(" function_formals_opt ")" type
                | expose_opt extern_opt "fn" identifier "(" function_formals_opt ")" type "->" funtion_body
function_formals_opt ::= function_formals | ""
function_formals ::= function_formals "," function_formal
                   | function_formal
function_formal ::= identifier mut_opt ref_opt type
funtion_body ::= rhs_expr


expose_opt ::= "expose" | ""
extern_opt ::= "extern" | ""


mut_opt ::= "mut" | ""
ref_opt ::= "ref" | ""
```

### Examples

Declaring the `writeln` function
```arvo
expose fn writeln(arg ref string) void -> __libprelude__writeln_string(arg)
extern fn __libprelude__writeln_string(arg ref string) void
```

Declaring the `+=` function
```arvo
expose fn (+=)(x mut ref i64, y i64) void -> {
  let tmp := deref x + y;
  x := tmp;
}
```

## Type declarations

```
type_decl ::= expose_opt enum_decl 
            | expose_opt struct_decl
type_params_opt ::= type_params | ""
type_params ::= type_params type_param
              | type_param
type_param ::= identifier


enum_decl ::= "type" identifier type_params_opt ":=" enum_variants
enum_variants ::= enum_variants "|" enum_variant 
                | enum_variant
enum_variant ::= identifier enum_variant_fields_opt
               | identifier "{" enum_variant_struct_fields "}"
enum_variant_fields_opt ::= enum_variant_fields | ""
enum_variant_fields ::= enum_variant_fields enum_variant_field
                      | enum_variant_field
enum_variant_field ::= type
enum_variant_struct_fields ::= enum_variant_struct_fields "," enum_variant_struct_field
                             | enum_variant_struct_field
enum_variant_struct_field ::= identifier type


struct_decl ::= "type" identifier type_params_opt "{" struct_fields_opt "}"
              | "type" identifier type_params_opt "{" struct_fields "," "}"
struct_fields_opt ::= struct_fields | ""
struct_fields ::= struct_fields "," struct_field
                | struct_field
struct_field ::= expose_opt identifier type


expose_opt ::= "expose" | ""
```

### Examples

Declarations for an `Option` type
```arvo
type Option a := Some a | Nil
type Option a := Some { expose inner a } | Nil
```

Declarations for a `Point` type
```arvo
type Point {
  x f64, y f64
}
type Point {
  x f64,
  y f64,
}
```

## Type symbols

```
type ::= channel_type
       | list_type
       | optional_type
       | tuple_type
       | unresolved_type
types_opt ::= types | ""
types ::= types type
               | type


channel_type ::= ".." type


list_type ::= "[" type "]"


optional_type ::= type "?"


tuple_type ::= "(" tuple_type_fields ")"
tuple_type_fields ::= tuple_type_fields "," tuple_type_field
                           | tuple_type_field
tuple_type_field ::= type


unresolved_type ::= identifier types_opt
```

### Examples

Declaring a type by instantiating the generic `Option` type that was declared
above
```arvo
Option i64
```

Declaring a point in space by instantiating the generic tuple
```arvo
(f64, f64, f64)
```

Declaring a list of integers by instantiating the generic list
```arvo
[i64]
```

Declaring a channel of points by instantiating the generic channel
```arvo
..(f64, f64, f64)
```

Declaring a channel of integers
```arvo
..i64
```

Declaring a channel of optional integers
```arvo
..i64?
```

Declaring an optional channel of integers
```arvo
(..i64)?
```

Declaring an optional channel of optional integers
```arvo
(..i64?)?
```

## Expressions

```
rhs_expr ::= assign_expr
           | block_expr
           | call_expr
           | channel_expr
           | deref_expr
           | for_expr
           | if_expr
           | item_expr
           | list_expr
           | literal_expr
           | operator_expr
           | ref_expr
           | select_expr
           | tuple_expr
           | "(" rhs_expr ")"
           | "(" ")"


assign_expr ::= lhs_pattern ":=" rhs_expr


block_expr ::= "{" block_expr_statements_opt rhs_expr "}"
block_expr_statements_opt ::= block_expr_statements | ""
block_expr_statements ::= block_expr_statements block_expr_statement
                        | block_expr_statement
block_expr_statement ::= let_expr ";"
                       | rhs_expr ";"
                       | ";"


call_expr ::= rhs_expr "(" call_expr_arguments_opt ")"
call_expr_arguments_opt ::= call_expr_arguments | ""
call_expr_arguments ::= call_expr_arguments "," call_expr_argument
                      | call_expr_argument
call_expr_argument ::= rhs_expr


channel_expr ::= ".." rhs_expr
               | rhs_expr ".." rhs_expr


deref_expr ::= "deref" rhs_expr


for_expr ::= "for" for_expr_items "in" for_expr_iterator block_expr
for_expr_items ::= lhs_pattern
                 | lhs_pattern "," identifier


if_expr ::= "if" if_expr_condition block_expr
          | "if" if_expr_condition block_expr "else" block_expr
          | "if" if_expr_condition block_expr "else" if_expr


item_expr ::= identifier
            | item_expr_path "." identifier
item_expr_path ::= rhs_expr


let_expr ::= "let" mut_opt lhs_pattern type ":=" rhs_expr


list_expr ::= "[" literal_tuple_expr_fields_opt "]"
list_expr_fields_opt ::= list_expr_fields | ""
list_expr_fields ::= list_expr_fields "," list_expr_field
                   | list_expr_field
list_expr_field ::= rhs_expr


literal_expr ::= literal_bool_expr
               | literal_char_expr
               | literal_float_expr
               | literal_integer_expr


operator_expr ::= rhs_expr binary_operator rhs_expr
                | prefix_operator rhs_expr
                | rhs_expr suffix_operator


ref_expr ::= "ref" rhs_expr


select_expr ::= "select" "{" select_expr_guards "}"
              | "select" "{" select_expr_guards "," "}" select_expr_else_opt
select_expr_guards ::= select_expr_guards "," select_expr_guard
select_expr_guards ::= "when" "<-" rhs_expr block_expr
                     | "when" "<-" rhs_expr "as" lhs_pattern block_expr
                     | "when" rhs_expr "<-" rhs_expr block_expr
select_expr_else_opt ::= select_expr_else | ""
select_expr_else ::= "else" block_expr

tuple_expr ::= "(" tuple_expr_fields ")"
tuple_expr_fields ::= tuple_expr_fields "," tuple_expr_field
                    | tuple_expr_field
tuple_expr_field ::= rhs_expr


mut_opt ::= "mut" | ""
```

### Examples

## Patterns

```
lhs_pattern ::= list_pattern
              | tuple_pattern
              | variable_pattern

list_pattern ::= "[" list_pattern_fields_opt "]"
list_pattern_fields_opt ::= list_pattern_fields | ""
list_pattern_fields ::= list_pattern_fields "," list_pattern_field
                      | list_pattern_field
list_pattern_field ::= lhs_pattern


tuple_pattern ::= "(" tuple_pattern_fields_opt ")"
tuple_pattern_fields ::= tuple_pattern_fields "," tuple_pattern_field
                      | tuple_pattern_field
tuple_pattern_field ::= lhs_pattern


variable_pattern ::= identifier
```

### Examples

## Comments

Comments are defined by regular expressions. Comments are omitted from the grammar, as they are permitted at the end of any line, including empty lines.

```
comment ::= //(.)*\n
```

### Examples

Any line that includes the double slash `//` will begin a line comment. If it includes a triple slash `///` then it still begins a line comment, but it will additionally document the node directly below (such comments are only allowed within a module).

```arvo
/// This comment documents the main module.
module main

/// This comment documents the main function.
fn main() void -> {
  // This comment does not document anything.
  writeln("Hello, world!"); // This will print "Hello, world!".
}
```

## Identifiers

Identifiers and operators are defined by regular expressions.

```
identifier ::= [_a-zA-Z]([_a-zA-Z]|[0-9])*
```

### Examples

## Literals

Literals are defined by regular expressions.

```
literal_bool_expr ::= (true|false)
literal_char_expr ::= '(\\'|[^']|\\u([0-9]|[ABCDEF]){4})?'
literal_float_expr ::= [0-9]+\.[0-9]+
literal_integer_expr ::= [0-9]+
literal_string_expr ::= "(\\"|[^"])*"
```

### Examples