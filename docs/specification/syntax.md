# Syntax Specification

> Work in progress.

## Module Declarations

A module declaration defines a new module. If parsing a file, the starting point is `decls_opt` and the surrounding module declaration is implicit (eeah file is considered a module of the same name, and contains module declarations at the top level).

```
module_decl ::= "module" module_name "{" decls_opt "}"
decls_opt ::= | decls
decls ::= decls decl
decl ::= function_decl | function_profile_decl | module_decl | type_decl
```

The `module_name` is the name of the module being declared.

## Function Declarations

A function declaration defines a new function profile with an optional definition of its behaviour.

```
function_decl ::= function_profile_decl "->" rhs_expr
function_profile_decl ::= expose_opt extern_opt "fn" function_name "(" formals_opt ")" return_type_opt
expose_opt ::= | "expose"
extern_opt ::= | "extern"
formals_opt ::= | formals
formals ::= formals "," formal
formal ::= formal_name type_identifier
return_type_opt ::= | type_identifier
```

* The `function_name` is the name of the function being declared.
* The `formal_name` is the name for a formal being declared.
* If the `return_type` of the function is omitted, it is assumed to be void.

## Type Declarations

A type declaration defines a new type.

```
type ::= type_struct | type_enum | type_alias
type_generics_opts ::= | type_generics
type_generics ::= type_generic_name | type_generics type_generic_name

type_struct ::= "type" type_name type_generics_opt "{"
    type_struct_fields_opt
    comma_opt
"}"
type_struct_fields_opt ::= | type_struct_fields
type_struct_fields ::= type_struct_field | type_struct_fields "," type_struct_field
type_struct_field ::= expose_opt type_field_name type_identifier
comma_opt ::= | ","
expose_opt ::= | "expose"

type_enum ::= "type" type_name type_generics_opt "=" type_enum_variants
type_enum_variants ::= type_enum_variant | type_enum_variants "|" type_enum_variant
type_enum_variant ::= type_name 
                    | type_name "{" type_enum_variant_fields comma_opt "}"
type_enum_variant_fields ::= type_enum_variant_field | type_enum_variant_fields "," type_enum_variant_field
type_enum_variant_field ::= type_field_name type_identifier

type_alias = "type" type_name type_generics_opt "=" type_identifier
```

* The `type_name` is the name of the type being declared.
* The `type_generic_name` is the name of a generic type within a type declaration.
* The `type_field_name` is the name of a type field within a type declaration.
* The use of `type_identifier` cannot include `ref` or `mut ref`.

```
// declaring a struct
type Point {
  x f64,
  y f64,
}

// declaring an enum
type Colors = Red | Green | Blue | RGB { r u8, g u8, b u8 }

// declaring a type alias
type Int64 = i64

// declaring a unit struct
type Unit {}
```

## Type Identifiers

A type identifier is used to access a type that is currently in scope.

```
type_identifier ::= identifier type_generic_identifiers_opt
                | "ref" identifier
                | "mut ref" identifier
                | "(" type_identifiers ")"

type_generic_identifiers_opts ::= | type_generic_identifiers
type_generic_identifiers ::= type_identifier | type_generic_identifiers type_identifier

type_identifiers ::= type_identifier | type_identifiers "," type_identifier
```

The definition of `type_identifier` uses `type_identifiers` to instantiate generic types.

## Expr

An expression is some behaviour that will result in a usable value.

```
expr ::= literal_expr | operator_expr | call_expr | item_expr | if_expr | block_expr | for_expr | "(" expr ")"

literal_expr ::= bool | char | float | integer | string | expr ".." expr | "[" literal_list_expr  "]"
literal_channel_expr ::= expr ".." expr
literal_list_expr ::= "[" literal_list_expr_exprs_opt "]"
literal_list_expr_exprs_opt ::= | literal_list_expr_exprs
literal_list_expr_exprs ::= expr | literal_list_expr_exprs "," expr

operator_expr ::= expr bin_operator expr | pre_operator expr
bin_operator ::= "+=" | "/=" | "*=" | "-=" | "+" | "-" | "/" | "*" | "=" | "<" | ">" | "<=" | ">="
pre_operator ::= "<-" | "!"

call_expr ::= call_target "(" call_argumets_opt ")"
call_target ::= item_expr | item_path_expr | call_expr
call_argumets_opt ::= | call_argumets
call_arguments ::= expr | call_argumets "," expr

item_expr ::= identifier
item_path_expr ::= expr "." identifier

if_expr ::= "if" expr block_expr
          | "if" expr block_expr "else" block_expr
          | "if" expr block_expr "else" if_expr

block_expr ::= "{" block_expr_exprs_opt semicolon_opt "}"
block_expr_exprs_opt ::= | block_expr_exprs
block_expr_exprs_opt ::= expr | block_expr_exprs ";" expr
semicolon_opt ::= | ";"

for_expr ::= "for" loop_name "in" expr block_expr
```

* All operators follow standard mathematical precedence and associativity.
* 