%{
#include <string.h>
#include "parser.h"

#define LITERAL(t) ({ yylval.literal = strdup(yytext); return t; })
#define TOKEN(t)   ({ yylval.token   = t;              return t; })
#define STRING(t)  ({                                        \
  char* literal = malloc(sizeof(char) * strlen(yytext) - 1); \
  strncpy(literal, &yytext[1], strlen(yytext)-2);            \
  literal[strlen(yytext)-1] = '\0';                          \
  yylval.literal = literal;                                  \
  return t;                                                  \
})

%}

%option noyywrap
%option yylineno

comment \/\/(.)*\n

digit  [0-9]
letter [_a-zA-Z]

bool  (true|false)
float {digit}*"."{digit}+
int   {digit}+

ident {letter}({letter}|{digit})*

%%

\n        {};
[ \t\r\f] {};

expose TOKEN(TEXPOSE);
extern TOKEN(TEXTERN);
fn     TOKEN(TFN);
module TOKEN(TMODULE);
mut    TOKEN(TMUT);
ref    TOKEN(TREF);
type   TOKEN(TTYPE);

";"  TOKEN(TSEMICOLON);
","  TOKEN(TCOMMA);
"."  TOKEN(TDOT);
"->" TOKEN(TRLAMBDA);

"(" TOKEN(TLPAREN);
")" TOKEN(TRPAREN);
"{" TOKEN(TLBRACE);
"}" TOKEN(TRBRACE);

"+"  LITERAL(TADD);
"/"  LITERAL(TDIV);
"^"  LITERAL(TEXP);
"*"  LITERAL(TMUL);
"-"  LITERAL(TSUB);

{comment} {};

{bool}  LITERAL(TBOOL);
{float} LITERAL(TFLOAT);
{int}   LITERAL(TINT);
{ident} LITERAL(TIDENT);

. { fprintf(stderr, "%d: error: unexpected token %s\n", yylineno, yytext); yyterminate(); }

%%
