%{

  // ----
  // yy globals and functions
  // ----

  extern FILE *yyin;
  extern int yylineno;

  extern int yylex();
  extern int yyparse();

  void yyerror(const char *msg) {
    fprintf(stderr, "%d: error: %s\n", yylineno, msg);
  }

  void yyopen(const char *filename) {
    yyin = fopen(filename, "r");
    if (!yyin) {
      fprintf(stderr, "error: cannot open file: %s\n", filename);
      return;
    }
    yypush_buffer_state(yy_create_buffer(yyin, YY_BUF_SIZE));
  }

  void yyclose() {
    fclose(yyin);
  }

  // ----
  // identifiers
  // ----

  struct Identifier {};
  struct Identifiers {};

  extern struct Identifiers* build_idents();
  extern struct Identifiers* push_ident(struct Identifiers* idents, struct Identifier* ident);

  extern struct Identifier* build_ident(const char* ident);

  // ----
  // nodes
  // ----

  struct Node {};

  extern struct Node* build_expr_node(struct Expr* expr);
  extern struct Node* build_item_node(struct Item* item);

  // ----
  // expressions
  // ----

  struct Expr {};
  struct Exprs {};

  extern struct Exprs* build_exprs();
  extern struct Exprs* push_expr(struct Exprs* exprs, struct Expr* expr);

  extern struct Expr* build_binary_op_expr(const char* op, struct Expr* lhs, struct Expr* rhs);
  extern struct Expr* build_block_expr(struct Exprs* block, struct Expr* definition);
  extern struct Expr* build_call_expr(struct Expr* target, struct Exprs* arguments);
  extern struct Expr* build_ident_expr(struct Identifier* ident);
  extern struct Expr* build_literal_bool_expr(const char* value);
  extern struct Expr* build_literal_float_expr(const char* value);
  extern struct Expr* build_literal_int_expr(const char* value);
  extern struct Expr* build_void_expr();

  // ----
  // items
  // ----

  struct Item {};
  struct Items {};

  extern struct Items* build_items();
  extern struct Items* push_item(struct Items* items, struct Item* item);

  extern struct Item* build_function(struct Item* function_profile, struct Expr* definition);
  extern struct Item* build_function_profile(struct Identifier* ident, struct Items* formals, struct Item* ty, int is_expose, int is_extern);
  extern struct Item* build_lambda_type(struct Items* formal_tys, struct Item* ty);
  extern struct Item* build_module(struct Identifier* ident, struct Items* items);
  extern struct Item* build_ref_type(struct Item* ty);
  extern struct Item* build_struct_type(struct Identifier* ident, struct Items* struct_fields, int is_expose);
  extern struct Item* build_unresolved_type(struct Identifier* ident);
  extern struct Item* build_variable(struct Identifier* ident, struct Item* ty, int is_mut, int is_expose);

  // ----
  // root node
  // ----

  struct Node* root;

%}

%union {
  int i;
  int token;
  char *literal;
  
  struct Identifier* identifier;
  struct Identifiers* identifiers;
  struct Node* node;
  struct Expr* expr;
  struct Exprs* exprs;
  struct Item* item;
  struct Items* items;
}

%token TCOMMA TDOT TEXPOSE TEXTERN TFN TMODULE TMUT TRBRACE TREF TRPAREN TSEMICOLON TTYPE

%token<literal> TBOOL TFLOAT TINT
%token<literal> TIDENT

%left<literal> TADD TSUB
%left<literal> TMUL TDIV
%left<literal> TEXP

%left<token> TLBRACE
%left<token> TLPAREN

%precedence<token> TRLAMBDA

// ----
// nodes
// ----

%type<node> top_level_node

// ----
// expressions
// ----

%type<expr> binary_op_expr
%type<expr> block_expr
%type<exprs> block_expr_body_exprs
%type<expr> block_expr_body_expr
%type<expr> call_expr
%type<exprs> opt_arguments
%type<exprs> arguments
%type<expr> argument
%type<expr> ident_expr
%type<expr> literal_expr
%type<expr> rhs_expr

// ----
// items
// ----

%type<item> function
%type<item> function_profile
%type<item> module
%type<items> opt_module_items
%type<items> module_items
%type<item> module_item
%type<items> opt_formals
%type<items> formals
%type<item> formal
%type<item> struct
%type<items> opt_struct_fields
%type<items> struct_fields
%type<item> struct_field
%type<item> ty

// ----
// misc
// ----

%type<identifier> ident
%type<identifier> op_ident
%type<i> opt_expose
%type<i> opt_extern
%type<i> opt_mut

%start top_level_node

%%

// ----
// node
// ----

top_level_node : opt_module_items { root = build_item_node(build_module(build_ident(""), $1)); }
               ;

// ----
// expressions
// ----

binary_op_expr : rhs_expr TADD rhs_expr { $$ = build_binary_op_expr($2, $1, $3); }
               | rhs_expr TDIV rhs_expr { $$ = build_binary_op_expr($2, $1, $3); }
               | rhs_expr TEXP rhs_expr { $$ = build_binary_op_expr($2, $1, $3); }
               | rhs_expr TMUL rhs_expr { $$ = build_binary_op_expr($2, $1, $3); }
               | rhs_expr TSUB rhs_expr { $$ = build_binary_op_expr($2, $1, $3); }
               ;

block_expr : TLBRACE                                TRBRACE { $$ = build_block_expr(build_exprs(), build_void_expr()); }
           | TLBRACE block_expr_body_exprs          TRBRACE { $$ = build_block_expr($2, build_void_expr()); }
           | TLBRACE                       rhs_expr TRBRACE { $$ = build_block_expr(build_exprs(), $2); }
           | TLBRACE block_expr_body_exprs rhs_expr TRBRACE { $$ = build_block_expr($2, $3); }
           ;

block_expr_body_exprs : block_expr_body_expr { $$ = push_expr(build_exprs(), $1); }
                      | block_expr_body_exprs block_expr_body_expr { $$ = push_expr($1, $2); }
                      ;

block_expr_body_expr : TSEMICOLON { $$ = build_void_expr(); }
                     | rhs_expr TSEMICOLON { $$ = $1; }
                     | block_expr
                     ;

call_expr : rhs_expr TLPAREN opt_arguments TRPAREN { $$ = build_call_expr($1, $3); }
          ;

opt_arguments : { $$ = build_exprs(); }
              | arguments
              ;

arguments : argument { $$ = push_expr(build_exprs(), $1); }
          | arguments TCOMMA argument { $$ = push_expr($1, $3); }
          ;

argument : rhs_expr
         ;

ident_expr : ident { $$ = build_ident_expr($1); }
           ;

literal_expr : TBOOL { $$ = build_literal_bool_expr($1); }
             | TFLOAT { $$ = build_literal_float_expr($1); }
             | TINT { $$ = build_literal_int_expr($1); } 
             ;

rhs_expr : binary_op_expr
         | call_expr
         | ident_expr
         | literal_expr
         | TLPAREN rhs_expr TRPAREN { $$ = $2; }
         ;
 
// ----
// items
// ----

function : function_profile TRLAMBDA rhs_expr { $$ = build_function($1, $3); }
         | function_profile TRLAMBDA block_expr { $$ = build_function($1, $3); }
         ;

function_profile : opt_expose opt_extern TFN ident TLPAREN opt_formals TRPAREN ty { $$ = build_function_profile($4, $6, $8, $1, $2); }
                 ;

opt_formals : { $$ = build_items(); }
            | formals
            ;

formals : formal { $$ = push_item(build_items(), $1); }
        | formals TCOMMA formal { $$ = push_item($1, $3); }
        ;

formal : opt_mut ident ty { $$ = build_variable($2, $3, $1, 0); }
       ;

module : TMODULE ident TLBRACE opt_module_items TRBRACE { $$ = build_module($2, $4); }
       ;

opt_module_items : { $$ = build_items(); }
                 | module_items
                 ;

module_items : module_item { $$ = push_item(build_items(), $1); }
             | module_items module_item { $$ = push_item($1, $2); }
             ;

module_item : function
            | function_profile
            | module
            | struct
            ;

struct : opt_expose TTYPE ident TLBRACE opt_struct_fields TRBRACE { $$ = build_struct_type($3, $5, $1); }
       ;

opt_struct_fields : { $$ = build_items(); }
                  | struct_fields
                  | struct_fields TCOMMA { $$ = $1; }
                  ;

struct_fields : struct_field { $$ = push_item(build_items(), $1); }
              | struct_fields TCOMMA struct_field { $$ = push_item($1, $3); }
              ;

struct_field : opt_expose ident ty { $$ = build_variable($2, $3, 1, $1); }
             ;

ty : ident { $$ = build_unresolved_type($1); }
   | TFN TLPAREN opt_formals TRPAREN ty { $$ = build_lambda_type($3, $5); }
   | TLPAREN ty TRPAREN { $$ = $2; }
   | TREF ty { $$ = build_ref_type($2); }
   ;

// ----
// misc
// ----

ident : op_ident
      | TIDENT { $$ = build_ident($1); }
      ;

op_ident : TLPAREN TADD TRPAREN { $$ = build_ident($2); }
         | TLPAREN TDIV TRPAREN { $$ = build_ident($2); }
         | TLPAREN TEXP TRPAREN { $$ = build_ident($2); }
         | TLPAREN TMUL TRPAREN { $$ = build_ident($2); }
         | TLPAREN TSUB TRPAREN { $$ = build_ident($2); }
         ;

opt_expose : {$$ = 0; }
           | TEXPOSE { $$ = 1; }
           ;

opt_extern : {$$ = 0; }
           | TEXTERN { $$ = 1; }
           ;

opt_mut : { $$ = 0; }
        | TMUT { $$ = 1; }
        ;

// // ----
// // modules
// // ----

// import_decl : TIMPORT import_idents { $$ = build_import_decl($2); }
//             ;

// import_idents : import_ident { $$ = push_ident(build_idents(), $1); }
//               | import_idents TDOT import_ident { $$ = push_ident($1, $3); }
//               ;

// import_ident : TIDENT { $$ = build_ident($1); }
//              ;

// module_decl : opt_module_inner_decls { root_module_decl = build_module_decl($1); }
//             ;

// opt_module_inner_decls : { $$ = build_decls(); }
//                        | module_inner_decls
//                        ;

// module_inner_decls : module_inner_decl { $$ = push_decl(build_decls(), $1); }
//                    | module_inner_decls module_inner_decl { $$ = push_decl($1, $2); }
//                    ;

// module_inner_decl : function_prototype_decl
//                   | function_decl
//                   | import_decl
//                   | type_decl
//                   ;

// // ----
// // functions
// // ----

// function_prototype_decl :                 TFN function_ident TLPAREN opt_formals TRPAREN ty { $$ = build_function_prototype_decl(0, 0, $2, $4, $6); }
//                         |         TEXTERN TFN function_ident TLPAREN opt_formals TRPAREN ty { $$ = build_function_prototype_decl(0, 1, $3, $5, $7); }
//                         | TEXPOSE         TFN function_ident TLPAREN opt_formals TRPAREN ty { $$ = build_function_prototype_decl(1, 0, $3, $5, $7); }
//                         | TEXPOSE TEXTERN TFN function_ident TLPAREN opt_formals TRPAREN ty { $$ = build_function_prototype_decl(1, 1, $4, $6, $8); }
//                         ;

// function_decl : function_prototype_decl TRLAMBDA expr { $$ = build_function_decl($1, $3); }

// opt_formals : { $$ = build_items(); }
//             | formals
//             ;

// formals : formal { $$ = push_item(build_items(), $1); }
//         | formals TCOMMA formal { $$ = push_item($1, $3); }
//         ;

// formal :      TIDENT opt_ty { $$ = build_variable($1, $2, 0, 0); }
//        | TMUT TIDENT opt_ty { $$ = build_variable($2, $3, 1, 0); }
//        ;

// function_ident : TIDENT
//                | operator
//                ;

// // ----
// // expressions
// // ----

// expr : assign_expr
//      | block_expr
//      | def_expr
//      | for_expr
//      | if_expr
//      | rhs_expr
//      ;

// rhs_expr : call_expr 
//          | ident_expr
//          | literal_expr
//          | operator_expr
//          | record_expr
//          | ref_expr
//          | TLPAREN rhs_expr TRPAREN { $$ = $2; }
//          ;

// lhs_expr : ident_expr
//          ;

// assign_expr : lhs_expr TASSIGN rhs_expr { $$ = build_assign_expr($1, $3); }
//             ;

// block_expr : TLBRACE                                 TRBRACE { $$ = build_block_expr(push_expr(build_exprs(), build_void_expr())); }
//            | TLBRACE block_expr_inner_exprs          TRBRACE { $$ = build_block_expr(push_expr($2, build_void_expr())); }
//            | TLBRACE                        rhs_expr TRBRACE { $$ = build_block_expr(push_expr(build_exprs(), $2)); }
//            | TLBRACE block_expr_inner_exprs rhs_expr TRBRACE { $$ = build_block_expr(push_expr($2, $3)); }
//            ;

// block_expr_inner_exprs : block_expr_inner_expr { $$ = push_expr(build_exprs(), $1); }
//                        | block_expr_inner_exprs block_expr_inner_expr { $$ = push_expr($1, $2); }
//                        ;

// block_expr_inner_expr :             TSEMICOLON { $$ = build_void_expr(); }
//                       | rhs_expr    TSEMICOLON { $$ = $1; }
//                       | assign_expr TSEMICOLON { $$ = $1; }
//                       | block_expr
//                       | def_expr    TSEMICOLON { $$ = $1; }
//                       | for_expr
//                       | if_expr
//                       ;

// call_expr : rhs_expr TLPAREN opt_arguments TRPAREN { $$ = build_call_expr($1, $3); }
//           ;

// opt_arguments : { $$ = build_exprs(); }
//               | arguments
//               ;

// arguments : rhs_expr { $$ = push_expr(build_exprs(), $1); }
//           | arguments TCOMMA rhs_expr { $$ = push_expr($1, $3); }
//           ;

// def_expr : TLET      pattern opt_ty TASSIGN block_expr { $$ = build_def_expr($2, $3, $5, 0); }
//          | TLET TMUT pattern opt_ty TASSIGN block_expr { $$ = build_def_expr($3, $4, $6, 1); }
//          | TLET      pattern opt_ty TASSIGN rhs_expr { $$ = build_def_expr($2, $3, $5, 0); }
//          | TLET TMUT pattern opt_ty TASSIGN rhs_expr { $$ = build_def_expr($3, $4, $6, 1); }
//          ;

// for_expr : TFOR pattern TIN rhs_expr TDO block_expr { $$ = build_for_expr($2, $4, $6); }
//          ;

// ident_expr : ident_path { $$ = build_ident_expr($1); }
//            ;

// ident_path : ident { $$ = push_ident(build_idents(), $1); }
//            | ident_path TDOT ident { $$ = push_ident($1, $3); }
//            ;

// ident : TIDENT { $$ = build_ident($1); }
//       ;

// if_expr : TIF rhs_expr TTHEN block_expr { $$ = build_if_expr($2, $4, build_void_expr()); }
//         | TIF rhs_expr TTHEN block_expr TELSE block_expr { $$ = build_if_expr($2, $4, $6); }
//         | TIF rhs_expr TTHEN block_expr TELSE if_expr { $$ = build_if_expr($2, $4, $6); }
//         ;

// literal_expr : TBOOLLIT { $$ = build_literal_bool_expr($1); }
//              | literal_expr TDOTDOT literal_expr { $$ = build_literal_channel_expr($1, $3); }
//              | TFLOATLIT { $$ = build_literal_f64_expr($1); }
//              | TINTLIT { $$ = build_literal_i64_expr($1); }
//              | TLBRACE literal_expr TDOTDOT literal_expr TRBRACE { $$ = build_literal_list_expr($2, $4); }
//              ;

// operator_expr : rhs_expr TPOW rhs_expr { $$ = build_operator_expr($2, push_expr(push_expr(build_exprs(), $1), $3)); }
//               | rhs_expr TMUL rhs_expr { $$ = build_operator_expr($2, push_expr(push_expr(build_exprs(), $1), $3)); }
//               | rhs_expr TDIV rhs_expr { $$ = build_operator_expr($2, push_expr(push_expr(build_exprs(), $1), $3)); }
//               | rhs_expr TADD rhs_expr { $$ = build_operator_expr($2, push_expr(push_expr(build_exprs(), $1), $3)); }
//               | rhs_expr TSUB rhs_expr { $$ = build_operator_expr($2, push_expr(push_expr(build_exprs(), $1), $3)); }
//               ;

// record_expr : ident_expr TLBRACE record_field_exprs TRBRACE { $$ = build_record_expr($1, $3); }
//             ;

// record_field_exprs : record_field_expr { $$ = push_expr(build_exprs(), $1); }
//                    | record_field_exprs TCOMMA record_field_expr { $$ = push_expr($1, $3); }
//                    ;

// record_field_expr : TIDENT TCOLON rhs_expr { $$ = build_record_field_expr($1, $3); }
//                   ;

// ref_expr : TREF rhs_expr { $$ = build_ref_expr($2, 0); }
//          | TMUT TREF rhs_expr { $$ = build_ref_expr($3, 1); }
//          ;

// // ----
// // types
// // ----

// type_decl : record_type_decl
//           ;

// record_type_decl : TTYPE TIDENT TLBRACE record_fields TRBRACE { $$ = build_record_type_decl($2, $4); }
//                  ;

// record_fields : record_field { $$ = push_item(build_items(), $1); }
//               | record_fields TCOMMA record_field { $$ = push_item($1, $3); }
//               ;

// record_field : TIDENT ty { $$ = build_variable($1, $2, 1); }
//              ;

// opt_tys : { $$ = build_items(); }
//         | tys
//         ;

// tys : ty { $$ = push_item(build_items(), $1); }
//     | tys TCOMMA ty { $$ = push_item($1, $3); }
//     ;

// opt_ty : { $$ = build_type(""); }
//        | ty
//        ;

// ty : ty_param
//    | TREF ty_param { $$ = build_ref_type($2, 0); }
//    | TMUT TREF ty_param { $$ = build_ref_type($3, 1); }
//    | TFN TLPAREN opt_tys TRPAREN ty { $$ = build_lambda_type($3, $5); }
//    ;

// ty_param : TIDENT { $$ = build_type($1); }
//          | TLPAREN ty TRPAREN { $$ = $2; }
//          ;

// // ----
// // patterns
// // ----

// pattern : ident_pattern
//         ;

// ident_pattern : ident { $$ = build_ident_pattern($1); }
//               ;