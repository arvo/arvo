# Syntax Specification

> Work in progress.

The grammar presented here is given in Backus–Naur form.

## Module Declarations

A module declaration defines a new module. If parsing a file, the starting point is `decls_opt` and the surrounding module declaration is implicit (eeah file is considered a module of the same name, and contains module declarations at the top level).

```
module_decl ::= "module" symbol "{" decls_opt "}"
decls_opt ::= | decls
decls ::= decls decl
decl ::= function_decl | function_profile_decl | module_decl | type_decl
```

## Function Declarations

A function declaration defines a new function profile with an optional definition of its behaviour.

```
function_decl ::= function_profile_decl "->" rhs_expr
function_profile_decl ::= expose_opt extern_opt "fn" symbol "(" formals_opt ")" return_type_opt
expose_opt ::= | "expose"
extern_opt ::= | "extern"
formals_opt ::= | formals
formals ::= formals "," formal
formal ::= symbol type_identifier
return_type_opt ::= | type_identifier
```

* If the `return_type` of the function is omitted, it is assumed to be void.

## Type Declarations

A type declaration defines a new type.

```
type ::= type_struct | type_enum | type_alias
type_params_opts ::= | type_params
type_params ::= type_param | type_params type_param
type_param ::= symbol

type_struct ::= "type" symbol type_params_opt "{"
    type_struct_fields_opt
    comma_opt
"}"
type_struct_fields_opt ::= | type_struct_fields
type_struct_fields ::= type_struct_field | type_struct_fields "," type_struct_field
type_struct_field ::= expose_opt symbol type_identifier
comma_opt ::= | ","
expose_opt ::= | "expose"

type_enum ::= "type" symbol type_params_opt "=" type_enum_variants
type_enum_variants ::= type_enum_variant | type_enum_variants "|" type_enum_variant
type_enum_variant ::= symbol type_params_opt
                    | symbol type_params_opt "{" type_enum_variant_fields comma_opt "}"
type_enum_variant_fields ::= type_enum_variant_field | type_enum_variant_fields "," type_enum_variant_field
type_enum_variant_field ::= symbol type_identifier

type_alias = "type" symbol type_params_opt "=" type_identifier
```

* The use of `type_identifier` cannot include `ref` or `mut ref`.

```
// declaring a struct
type Point {
  x f64,
  y f64,
}

// declaring a generic struct
type List a {
  items [a],
}

// declaring an enum
type Colors = Red | Green | Blue | RGB { r u8, g u8, b u8 }

// declaring a generic enum
type Option a = Some a | Nil

// declaring a type alias
type Int64 = i64

// declaring a unit struct
type Unit {}
```

## Type Identifiers

A type identifier is used to access a type that is currently in scope.

```
type_identifier ::= symbol type_param_identifiers_opt
                | "ref" type_identifier
                | "mut ref" type_identifier
                | "(" type_identifiers ")"

type_param_identifiers_opts ::= | type_param_identifiers
type_param_identifiers ::= type_identifier | type_param_identifiers type_identifier

type_identifiers ::= type_identifier | type_identifiers "," type_identifier
```

* The definition of `type_identifier` includes the instantiation of generic type parameters.

## Expr

An expression is some behaviour that will result in a usable value.

```
expr ::= literal_expr | operator_expr | call_expr | item_path_expr | if_expr | block_expr | let_expr | for_expr | ref_expr | deref_expr | assign_expr | "(" expr ")"
expr_opt ::= | expr

literal_expr ::= bool | char | float | integer | string | literal_channel_expr | literal_list_expr
literal_channel_expr ::= expr ".." expr
literal_list_expr ::= "[" literal_list_expr_exprs_opt "]" | "[" expr ".." expr "]"
literal_list_expr_exprs_opt ::= | literal_list_expr_exprs
literal_list_expr_exprs ::= expr | literal_list_expr_exprs "," expr

operator_expr ::= expr bin_operator expr | pre_operator expr
bin_operator ::= "+=" | "/=" | "*=" | "-=" | "+" | "-" | "/" | "*" | "=" | "<" | ">" | "<=" | ">="
pre_operator ::= "<-" | "!" | "-"

call_expr ::= call_target "(" call_argumets_opt ")"
call_target ::= expr
call_argumets_opt ::= | call_argumets
call_arguments ::= expr | call_argumets "," expr

item_path_expr ::= symbol | expr "." symbol

if_expr ::= "if" if_condition block_expr
          | "if" if_condition block_expr "else" block_expr
          | "if" if_condition block_expr "else" if_expr
if_condition ::= expr

block_expr ::= "{" block_expr_exprs_opt expr_opt "}"
block_expr_exprs_opt ::= | block_expr_exprs ";"
block_expr_exprs ::= block_expr_expr | block_expr_exprs ";" block_expr_expr
block_expr_expr ::= let_expr | expr

let_expr ::= "let" mut_opt symbol ":=" expr

for_expr ::= "for" for_formals "in" expr block_expr
for_formals ::= for_formal | for_formals "," for_formal
for_formal ::= symbol

ref_expr ::= "ref" expr
deref_expr ::= "deref" expr

assign_expr ::= symbol ":=" expr | "deref" symbol ":=" expr
```

* All operators follow standard mathematical precedence and associativity.
* The `expr_opt` defaults to a void expression.
