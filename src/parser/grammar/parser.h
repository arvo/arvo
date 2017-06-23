/* A Bison parser, made by GNU Bison 3.0.4.  */

/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "3.0.4"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1




/* Copy the first part of user declarations.  */
#line 1 "./grammar/parser.y" /* yacc.c:339  */


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


#line 162 "./grammar/parser.h" /* yacc.c:339  */

# ifndef YY_NULLPTR
#  if defined __cplusplus && 201103L <= __cplusplus
#   define YY_NULLPTR nullptr
#  else
#   define YY_NULLPTR 0
#  endif
# endif

/* Enabling verbose error messages.  */
#ifdef YYERROR_VERBOSE
# undef YYERROR_VERBOSE
# define YYERROR_VERBOSE 1
#else
# define YYERROR_VERBOSE 0
#endif


/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token type.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    TCOMMA = 258,
    TDOT = 259,
    TEXPOSE = 260,
    TEXTERN = 261,
    TFN = 262,
    TMODULE = 263,
    TMUT = 264,
    TRBRACE = 265,
    TREF = 266,
    TRPAREN = 267,
    TSEMICOLON = 268,
    TTYPE = 269,
    TBOOL = 270,
    TFLOAT = 271,
    TINT = 272,
    TIDENT = 273,
    TADD = 274,
    TSUB = 275,
    TMUL = 276,
    TDIV = 277,
    TEXP = 278,
    TLBRACE = 279,
    TLPAREN = 280,
    TRLAMBDA = 281
  };
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED

union YYSTYPE
{
#line 97 "./grammar/parser.y" /* yacc.c:355  */

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

#line 240 "./grammar/parser.h" /* yacc.c:355  */
};

typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;

int yyparse (void);



/* Copy the second part of user declarations.  */

#line 257 "./grammar/parser.h" /* yacc.c:358  */

#ifdef short
# undef short
#endif

#ifdef YYTYPE_UINT8
typedef YYTYPE_UINT8 yytype_uint8;
#else
typedef unsigned char yytype_uint8;
#endif

#ifdef YYTYPE_INT8
typedef YYTYPE_INT8 yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef YYTYPE_UINT16
typedef YYTYPE_UINT16 yytype_uint16;
#else
typedef unsigned short int yytype_uint16;
#endif

#ifdef YYTYPE_INT16
typedef YYTYPE_INT16 yytype_int16;
#else
typedef short int yytype_int16;
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif ! defined YYSIZE_T
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned int
# endif
#endif

#define YYSIZE_MAXIMUM ((YYSIZE_T) -1)

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif

#ifndef YY_ATTRIBUTE
# if (defined __GNUC__                                               \
      && (2 < __GNUC__ || (__GNUC__ == 2 && 96 <= __GNUC_MINOR__)))  \
     || defined __SUNPRO_C && 0x5110 <= __SUNPRO_C
#  define YY_ATTRIBUTE(Spec) __attribute__(Spec)
# else
#  define YY_ATTRIBUTE(Spec) /* empty */
# endif
#endif

#ifndef YY_ATTRIBUTE_PURE
# define YY_ATTRIBUTE_PURE   YY_ATTRIBUTE ((__pure__))
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# define YY_ATTRIBUTE_UNUSED YY_ATTRIBUTE ((__unused__))
#endif

#if !defined _Noreturn \
     && (!defined __STDC_VERSION__ || __STDC_VERSION__ < 201112)
# if defined _MSC_VER && 1200 <= _MSC_VER
#  define _Noreturn __declspec (noreturn)
# else
#  define _Noreturn YY_ATTRIBUTE ((__noreturn__))
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN \
    _Pragma ("GCC diagnostic push") \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")\
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif


#if ! defined yyoverflow || YYERROR_VERBOSE

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* ! defined yyoverflow || YYERROR_VERBOSE */


#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yytype_int16 yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (sizeof (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (sizeof (yytype_int16) + sizeof (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYSIZE_T yynewbytes;                                            \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * sizeof (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / sizeof (*yyptr);                          \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, (Count) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYSIZE_T yyi;                         \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  16
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   167

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  27
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  32
/* YYNRULES -- Number of rules.  */
#define YYNRULES  72
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  115

/* YYTRANSLATE[YYX] -- Symbol number corresponding to YYX as returned
   by yylex, with out-of-bounds checking.  */
#define YYUNDEFTOK  2
#define YYMAXUTOK   281

#define YYTRANSLATE(YYX)                                                \
  ((unsigned int) (YYX) <= YYMAXUTOK ? yytranslate[YYX] : YYUNDEFTOK)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, without out-of-bounds checking.  */
static const yytype_uint8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_uint16 yyrline[] =
{
       0,   184,   184,   191,   192,   193,   194,   195,   198,   199,
     200,   201,   204,   205,   208,   209,   210,   213,   216,   217,
     220,   221,   224,   227,   230,   231,   232,   235,   236,   237,
     238,   239,   246,   247,   250,   253,   254,   257,   258,   261,
     264,   267,   268,   271,   272,   275,   276,   277,   278,   281,
     284,   285,   286,   289,   290,   293,   296,   297,   298,   299,
     306,   307,   310,   311,   312,   313,   314,   317,   318,   321,
     322,   325,   326
};
#endif

#if YYDEBUG || YYERROR_VERBOSE || 0
/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "$end", "error", "$undefined", "TCOMMA", "TDOT", "TEXPOSE", "TEXTERN",
  "TFN", "TMODULE", "TMUT", "TRBRACE", "TREF", "TRPAREN", "TSEMICOLON",
  "TTYPE", "TBOOL", "TFLOAT", "TINT", "TIDENT", "TADD", "TSUB", "TMUL",
  "TDIV", "TEXP", "TLBRACE", "TLPAREN", "TRLAMBDA", "$accept",
  "top_level_node", "binary_op_expr", "block_expr",
  "block_expr_body_exprs", "block_expr_body_expr", "call_expr",
  "opt_arguments", "arguments", "argument", "ident_expr", "literal_expr",
  "rhs_expr", "function", "function_profile", "opt_formals", "formals",
  "formal", "module", "opt_module_items", "module_items", "module_item",
  "struct", "opt_struct_fields", "struct_fields", "struct_field", "ty",
  "ident", "op_ident", "opt_expose", "opt_extern", "opt_mut", YY_NULLPTR
};
#endif

# ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_uint16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,   278,   279,   280,   281
};
# endif

#define YYPACT_NINF -61

#define yypact_value_is_default(Yystate) \
  (!!((Yystate) == (-61)))

#define YYTABLE_NINF -53

#define yytable_value_is_error(Yytable_value) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
      29,   -61,    30,     9,   -61,     6,   -61,   -61,    25,   -61,
     -61,    -3,   -61,    94,    32,   -61,   -61,   -11,   -61,   -61,
      30,    50,    57,    64,    67,    77,    81,    80,   -61,   -61,
     -61,    36,   122,   -61,   -61,   -61,   -61,   -61,   129,   -61,
      54,    30,   -61,   -61,   -61,   -61,   -61,    84,   -61,   -61,
     -61,    49,   -61,    85,   111,    66,    66,    66,    66,    66,
      66,     5,    74,   -61,   -61,   -61,    99,   -61,   -61,   -61,
     104,   104,    45,    45,    76,    91,   125,   -61,   129,   136,
     132,   -61,    30,    38,   -61,   -61,    66,   -61,    53,    20,
     -61,   141,   152,   -61,    30,   -61,   -61,   131,    20,     1,
     -61,   -61,    20,   148,    20,    38,   -61,   146,   -61,   -61,
     -61,   147,   -61,    20,   -61
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_uint8 yydefact[] =
{
      67,    68,     0,     0,    45,    46,    47,     2,    67,    43,
      48,    69,    61,     0,     0,    60,     1,     0,    44,    70,
       0,     0,     0,     0,     0,     0,     0,    67,    24,    25,
      26,     0,     0,    27,    33,    28,    29,    30,    32,    23,
       0,     0,    62,    66,    65,    63,    64,     0,     8,    14,
      16,     0,    12,     0,     0,     0,     0,     0,     0,     0,
      18,    67,     0,    40,     9,    13,     0,    10,    15,    31,
       3,     7,     6,     4,     5,     0,    19,    20,    22,     0,
      51,    53,     0,    71,    11,    17,     0,    49,    67,     0,
      72,     0,    36,    37,     0,    21,    54,     0,     0,     0,
      55,    56,     0,    71,     0,    71,    59,     0,    34,    38,
      39,     0,    58,     0,    57
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
     -61,   -61,   -61,   143,   -61,   110,   -61,   -61,   -61,    78,
     -61,   -61,   -15,   -61,   -61,    58,   -61,    59,   -61,   138,
     -61,   158,   -61,   -61,   -61,    79,   -27,    -2,   -61,   -60,
     -61,   -61
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int8 yydefgoto[] =
{
      -1,     3,    33,    50,    51,    52,    35,    75,    76,    77,
      36,    37,    78,     4,     5,    91,    92,    93,     6,     7,
       8,     9,    10,    79,    80,    81,   100,    39,    15,    11,
      21,    94
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int8 yytable[] =
{
      14,    82,    38,    19,    28,    29,    30,    12,    97,    16,
       1,    20,    98,    31,    32,   -50,    53,    54,    40,    12,
      22,    23,    24,    25,    26,   -42,    99,    97,    82,   -41,
       1,    98,    17,     2,     1,   -42,    66,     2,    12,    62,
      70,    71,    72,    73,    74,    99,    48,    90,    12,    49,
     -35,    28,    29,    30,    12,    13,    27,    41,     1,    64,
      31,    32,    49,   -52,    28,    29,    30,    12,    59,    42,
      60,   106,   107,    31,    32,   108,    43,   110,    61,    44,
      89,    28,    29,    30,    12,     1,   114,   101,     2,    45,
     -41,    32,   104,    46,    63,    67,   101,   101,    68,    83,
     101,    60,   101,    85,    55,    56,    57,    58,    59,    84,
      60,   101,    68,    22,    23,    24,    25,    26,    55,    56,
      57,    58,    59,    69,    60,    57,    58,    59,    86,    60,
      55,    56,    57,    58,    59,    88,    60,    28,    29,    30,
      12,    22,    23,    24,    25,    26,    87,    32,    55,    56,
      57,    58,    59,   102,    60,   103,   105,    90,   112,   113,
      34,    65,   109,   111,    95,    47,    18,    96
};

static const yytype_uint8 yycheck[] =
{
       2,    61,    17,     6,    15,    16,    17,    18,     7,     0,
       5,    14,    11,    24,    25,    10,    31,    32,    20,    18,
      19,    20,    21,    22,    23,     0,    25,     7,    88,     0,
       5,    11,    26,     8,     5,    10,    51,     8,    18,    41,
      55,    56,    57,    58,    59,    25,    10,     9,    18,    13,
      12,    15,    16,    17,    18,    25,    24,     7,     5,    10,
      24,    25,    13,    10,    15,    16,    17,    18,    23,    12,
      25,    98,    99,    24,    25,   102,    12,   104,    24,    12,
      82,    15,    16,    17,    18,     5,   113,    89,     8,    12,
      10,    25,    94,    12,    10,    10,    98,    99,    13,    25,
     102,    25,   104,    12,    19,    20,    21,    22,    23,    10,
      25,   113,    13,    19,    20,    21,    22,    23,    19,    20,
      21,    22,    23,    12,    25,    21,    22,    23,     3,    25,
      19,    20,    21,    22,    23,     3,    25,    15,    16,    17,
      18,    19,    20,    21,    22,    23,    10,    25,    19,    20,
      21,    22,    23,    12,    25,     3,    25,     9,    12,    12,
      17,    51,   103,   105,    86,    27,     8,    88
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_uint8 yystos[] =
{
       0,     5,     8,    28,    40,    41,    45,    46,    47,    48,
      49,    56,    18,    25,    54,    55,     0,    26,    48,     6,
      14,    57,    19,    20,    21,    22,    23,    24,    15,    16,
      17,    24,    25,    29,    30,    33,    37,    38,    39,    54,
      54,     7,    12,    12,    12,    12,    12,    46,    10,    13,
      30,    31,    32,    39,    39,    19,    20,    21,    22,    23,
      25,    24,    54,    10,    10,    32,    39,    10,    13,    12,
      39,    39,    39,    39,    39,    34,    35,    36,    39,    50,
      51,    52,    56,    25,    10,    12,     3,    10,     3,    54,
       9,    42,    43,    44,    58,    36,    52,     7,    11,    25,
      53,    54,    12,     3,    54,    25,    53,    53,    53,    44,
      53,    42,    12,    12,    53
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_uint8 yyr1[] =
{
       0,    27,    28,    29,    29,    29,    29,    29,    30,    30,
      30,    30,    31,    31,    32,    32,    32,    33,    34,    34,
      35,    35,    36,    37,    38,    38,    38,    39,    39,    39,
      39,    39,    40,    40,    41,    42,    42,    43,    43,    44,
      45,    46,    46,    47,    47,    48,    48,    48,    48,    49,
      50,    50,    50,    51,    51,    52,    53,    53,    53,    53,
      54,    54,    55,    55,    55,    55,    55,    56,    56,    57,
      57,    58,    58
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_uint8 yyr2[] =
{
       0,     2,     1,     3,     3,     3,     3,     3,     2,     3,
       3,     4,     1,     2,     1,     2,     1,     4,     0,     1,
       1,     3,     1,     1,     1,     1,     1,     1,     1,     1,
       1,     3,     3,     3,     8,     0,     1,     1,     3,     3,
       5,     0,     1,     1,     2,     1,     1,     1,     1,     6,
       0,     1,     2,     1,     3,     3,     1,     5,     3,     2,
       1,     1,     3,     3,     3,     3,     3,     0,     1,     0,
       1,     0,     1
};


#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)
#define YYEMPTY         (-2)
#define YYEOF           0

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                  \
do                                                              \
  if (yychar == YYEMPTY)                                        \
    {                                                           \
      yychar = (Token);                                         \
      yylval = (Value);                                         \
      YYPOPSTACK (yylen);                                       \
      yystate = *yyssp;                                         \
      goto yybackup;                                            \
    }                                                           \
  else                                                          \
    {                                                           \
      yyerror (YY_("syntax error: cannot back up")); \
      YYERROR;                                                  \
    }                                                           \
while (0)

/* Error token number */
#define YYTERROR        1
#define YYERRCODE       256



/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)

/* This macro is provided for backward compatibility. */
#ifndef YY_LOCATION_PRINT
# define YY_LOCATION_PRINT(File, Loc) ((void) 0)
#endif


# define YY_SYMBOL_PRINT(Title, Type, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Type, Value); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*----------------------------------------.
| Print this symbol's value on YYOUTPUT.  |
`----------------------------------------*/

static void
yy_symbol_value_print (FILE *yyoutput, int yytype, YYSTYPE const * const yyvaluep)
{
  FILE *yyo = yyoutput;
  YYUSE (yyo);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yytype < YYNTOKENS)
    YYPRINT (yyoutput, yytoknum[yytype], *yyvaluep);
# endif
  YYUSE (yytype);
}


/*--------------------------------.
| Print this symbol on YYOUTPUT.  |
`--------------------------------*/

static void
yy_symbol_print (FILE *yyoutput, int yytype, YYSTYPE const * const yyvaluep)
{
  YYFPRINTF (yyoutput, "%s %s (",
             yytype < YYNTOKENS ? "token" : "nterm", yytname[yytype]);

  yy_symbol_value_print (yyoutput, yytype, yyvaluep);
  YYFPRINTF (yyoutput, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yytype_int16 *yybottom, yytype_int16 *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yytype_int16 *yyssp, YYSTYPE *yyvsp, int yyrule)
{
  unsigned long int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %lu):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       yystos[yyssp[yyi + 1 - yynrhs]],
                       &(yyvsp[(yyi + 1) - (yynrhs)])
                                              );
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args)
# define YY_SYMBOL_PRINT(Title, Type, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif


#if YYERROR_VERBOSE

# ifndef yystrlen
#  if defined __GLIBC__ && defined _STRING_H
#   define yystrlen strlen
#  else
/* Return the length of YYSTR.  */
static YYSIZE_T
yystrlen (const char *yystr)
{
  YYSIZE_T yylen;
  for (yylen = 0; yystr[yylen]; yylen++)
    continue;
  return yylen;
}
#  endif
# endif

# ifndef yystpcpy
#  if defined __GLIBC__ && defined _STRING_H && defined _GNU_SOURCE
#   define yystpcpy stpcpy
#  else
/* Copy YYSRC to YYDEST, returning the address of the terminating '\0' in
   YYDEST.  */
static char *
yystpcpy (char *yydest, const char *yysrc)
{
  char *yyd = yydest;
  const char *yys = yysrc;

  while ((*yyd++ = *yys++) != '\0')
    continue;

  return yyd - 1;
}
#  endif
# endif

# ifndef yytnamerr
/* Copy to YYRES the contents of YYSTR after stripping away unnecessary
   quotes and backslashes, so that it's suitable for yyerror.  The
   heuristic is that double-quoting is unnecessary unless the string
   contains an apostrophe, a comma, or backslash (other than
   backslash-backslash).  YYSTR is taken from yytname.  If YYRES is
   null, do not copy; instead, return the length of what the result
   would have been.  */
static YYSIZE_T
yytnamerr (char *yyres, const char *yystr)
{
  if (*yystr == '"')
    {
      YYSIZE_T yyn = 0;
      char const *yyp = yystr;

      for (;;)
        switch (*++yyp)
          {
          case '\'':
          case ',':
            goto do_not_strip_quotes;

          case '\\':
            if (*++yyp != '\\')
              goto do_not_strip_quotes;
            /* Fall through.  */
          default:
            if (yyres)
              yyres[yyn] = *yyp;
            yyn++;
            break;

          case '"':
            if (yyres)
              yyres[yyn] = '\0';
            return yyn;
          }
    do_not_strip_quotes: ;
    }

  if (! yyres)
    return yystrlen (yystr);

  return yystpcpy (yyres, yystr) - yyres;
}
# endif

/* Copy into *YYMSG, which is of size *YYMSG_ALLOC, an error message
   about the unexpected token YYTOKEN for the state stack whose top is
   YYSSP.

   Return 0 if *YYMSG was successfully written.  Return 1 if *YYMSG is
   not large enough to hold the message.  In that case, also set
   *YYMSG_ALLOC to the required number of bytes.  Return 2 if the
   required number of bytes is too large to store.  */
static int
yysyntax_error (YYSIZE_T *yymsg_alloc, char **yymsg,
                yytype_int16 *yyssp, int yytoken)
{
  YYSIZE_T yysize0 = yytnamerr (YY_NULLPTR, yytname[yytoken]);
  YYSIZE_T yysize = yysize0;
  enum { YYERROR_VERBOSE_ARGS_MAXIMUM = 5 };
  /* Internationalized format string. */
  const char *yyformat = YY_NULLPTR;
  /* Arguments of yyformat. */
  char const *yyarg[YYERROR_VERBOSE_ARGS_MAXIMUM];
  /* Number of reported tokens (one for the "unexpected", one per
     "expected"). */
  int yycount = 0;

  /* There are many possibilities here to consider:
     - If this state is a consistent state with a default action, then
       the only way this function was invoked is if the default action
       is an error action.  In that case, don't check for expected
       tokens because there are none.
     - The only way there can be no lookahead present (in yychar) is if
       this state is a consistent state with a default action.  Thus,
       detecting the absence of a lookahead is sufficient to determine
       that there is no unexpected or expected token to report.  In that
       case, just report a simple "syntax error".
     - Don't assume there isn't a lookahead just because this state is a
       consistent state with a default action.  There might have been a
       previous inconsistent state, consistent state with a non-default
       action, or user semantic action that manipulated yychar.
     - Of course, the expected token list depends on states to have
       correct lookahead information, and it depends on the parser not
       to perform extra reductions after fetching a lookahead from the
       scanner and before detecting a syntax error.  Thus, state merging
       (from LALR or IELR) and default reductions corrupt the expected
       token list.  However, the list is correct for canonical LR with
       one exception: it will still contain any token that will not be
       accepted due to an error action in a later state.
  */
  if (yytoken != YYEMPTY)
    {
      int yyn = yypact[*yyssp];
      yyarg[yycount++] = yytname[yytoken];
      if (!yypact_value_is_default (yyn))
        {
          /* Start YYX at -YYN if negative to avoid negative indexes in
             YYCHECK.  In other words, skip the first -YYN actions for
             this state because they are default actions.  */
          int yyxbegin = yyn < 0 ? -yyn : 0;
          /* Stay within bounds of both yycheck and yytname.  */
          int yychecklim = YYLAST - yyn + 1;
          int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
          int yyx;

          for (yyx = yyxbegin; yyx < yyxend; ++yyx)
            if (yycheck[yyx + yyn] == yyx && yyx != YYTERROR
                && !yytable_value_is_error (yytable[yyx + yyn]))
              {
                if (yycount == YYERROR_VERBOSE_ARGS_MAXIMUM)
                  {
                    yycount = 1;
                    yysize = yysize0;
                    break;
                  }
                yyarg[yycount++] = yytname[yyx];
                {
                  YYSIZE_T yysize1 = yysize + yytnamerr (YY_NULLPTR, yytname[yyx]);
                  if (! (yysize <= yysize1
                         && yysize1 <= YYSTACK_ALLOC_MAXIMUM))
                    return 2;
                  yysize = yysize1;
                }
              }
        }
    }

  switch (yycount)
    {
# define YYCASE_(N, S)                      \
      case N:                               \
        yyformat = S;                       \
      break
      YYCASE_(0, YY_("syntax error"));
      YYCASE_(1, YY_("syntax error, unexpected %s"));
      YYCASE_(2, YY_("syntax error, unexpected %s, expecting %s"));
      YYCASE_(3, YY_("syntax error, unexpected %s, expecting %s or %s"));
      YYCASE_(4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
      YYCASE_(5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
# undef YYCASE_
    }

  {
    YYSIZE_T yysize1 = yysize + yystrlen (yyformat);
    if (! (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM))
      return 2;
    yysize = yysize1;
  }

  if (*yymsg_alloc < yysize)
    {
      *yymsg_alloc = 2 * yysize;
      if (! (yysize <= *yymsg_alloc
             && *yymsg_alloc <= YYSTACK_ALLOC_MAXIMUM))
        *yymsg_alloc = YYSTACK_ALLOC_MAXIMUM;
      return 1;
    }

  /* Avoid sprintf, as that infringes on the user's name space.
     Don't have undefined behavior even if the translation
     produced a string with the wrong number of "%s"s.  */
  {
    char *yyp = *yymsg;
    int yyi = 0;
    while ((*yyp = *yyformat) != '\0')
      if (*yyp == '%' && yyformat[1] == 's' && yyi < yycount)
        {
          yyp += yytnamerr (yyp, yyarg[yyi++]);
          yyformat += 2;
        }
      else
        {
          yyp++;
          yyformat++;
        }
  }
  return 0;
}
#endif /* YYERROR_VERBOSE */

/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg, int yytype, YYSTYPE *yyvaluep)
{
  YYUSE (yyvaluep);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yytype, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yytype);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}




/* The lookahead symbol.  */
int yychar;

/* The semantic value of the lookahead symbol.  */
YYSTYPE yylval;
/* Number of syntax errors so far.  */
int yynerrs;


/*----------.
| yyparse.  |
`----------*/

int
yyparse (void)
{
    int yystate;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus;

    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* The state stack.  */
    yytype_int16 yyssa[YYINITDEPTH];
    yytype_int16 *yyss;
    yytype_int16 *yyssp;

    /* The semantic value stack.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs;
    YYSTYPE *yyvsp;

    YYSIZE_T yystacksize;

  int yyn;
  int yyresult;
  /* Lookahead token as an internal (translated) token number.  */
  int yytoken = 0;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;

#if YYERROR_VERBOSE
  /* Buffer for error messages, and its allocated size.  */
  char yymsgbuf[128];
  char *yymsg = yymsgbuf;
  YYSIZE_T yymsg_alloc = sizeof yymsgbuf;
#endif

#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  yyssp = yyss = yyssa;
  yyvsp = yyvs = yyvsa;
  yystacksize = YYINITDEPTH;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yystate = 0;
  yyerrstatus = 0;
  yynerrs = 0;
  yychar = YYEMPTY; /* Cause a token to be read.  */
  goto yysetstate;

/*------------------------------------------------------------.
| yynewstate -- Push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
 yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;

 yysetstate:
  *yyssp = yystate;

  if (yyss + yystacksize - 1 <= yyssp)
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYSIZE_T yysize = yyssp - yyss + 1;

#ifdef yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        YYSTYPE *yyvs1 = yyvs;
        yytype_int16 *yyss1 = yyss;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * sizeof (*yyssp),
                    &yyvs1, yysize * sizeof (*yyvsp),
                    &yystacksize);

        yyss = yyss1;
        yyvs = yyvs1;
      }
#else /* no yyoverflow */
# ifndef YYSTACK_RELOCATE
      goto yyexhaustedlab;
# else
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yytype_int16 *yyss1 = yyss;
        union yyalloc *yyptr =
          (union yyalloc *) YYSTACK_ALLOC (YYSTACK_BYTES (yystacksize));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
#  undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif
#endif /* no yyoverflow */

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YYDPRINTF ((stderr, "Stack size increased to %lu\n",
                  (unsigned long int) yystacksize));

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }

  YYDPRINTF ((stderr, "Entering state %d\n", yystate));

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;

/*-----------.
| yybackup.  |
`-----------*/
yybackup:

  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token: "));
      yychar = yylex ();
    }

  if (yychar <= YYEOF)
    {
      yychar = yytoken = YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);

  /* Discard the shifted token.  */
  yychar = YYEMPTY;

  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- Do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
        case 2:
#line 184 "./grammar/parser.y" /* yacc.c:1646  */
    { root = build_item_node(build_module(build_ident(""), (yyvsp[0].items))); }
#line 1428 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 3:
#line 191 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_binary_op_expr((yyvsp[-1].literal), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 1434 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 4:
#line 192 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_binary_op_expr((yyvsp[-1].literal), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 1440 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 5:
#line 193 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_binary_op_expr((yyvsp[-1].literal), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 1446 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 6:
#line 194 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_binary_op_expr((yyvsp[-1].literal), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 1452 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 7:
#line 195 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_binary_op_expr((yyvsp[-1].literal), (yyvsp[-2].expr), (yyvsp[0].expr)); }
#line 1458 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 8:
#line 198 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_block_expr(build_exprs(), build_void_expr()); }
#line 1464 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 9:
#line 199 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_block_expr((yyvsp[-1].exprs), build_void_expr()); }
#line 1470 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 10:
#line 200 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_block_expr(build_exprs(), (yyvsp[-1].expr)); }
#line 1476 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 11:
#line 201 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_block_expr((yyvsp[-2].exprs), (yyvsp[-1].expr)); }
#line 1482 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 12:
#line 204 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.exprs) = push_expr(build_exprs(), (yyvsp[0].expr)); }
#line 1488 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 13:
#line 205 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.exprs) = push_expr((yyvsp[-1].exprs), (yyvsp[0].expr)); }
#line 1494 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 14:
#line 208 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_void_expr(); }
#line 1500 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 15:
#line 209 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = (yyvsp[-1].expr); }
#line 1506 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 17:
#line 213 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_call_expr((yyvsp[-3].expr), (yyvsp[-1].exprs)); }
#line 1512 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 18:
#line 216 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.exprs) = build_exprs(); }
#line 1518 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 20:
#line 220 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.exprs) = push_expr(build_exprs(), (yyvsp[0].expr)); }
#line 1524 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 21:
#line 221 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.exprs) = push_expr((yyvsp[-2].exprs), (yyvsp[0].expr)); }
#line 1530 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 23:
#line 227 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_ident_expr((yyvsp[0].identifier)); }
#line 1536 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 24:
#line 230 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_literal_bool_expr((yyvsp[0].literal)); }
#line 1542 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 25:
#line 231 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_literal_float_expr((yyvsp[0].literal)); }
#line 1548 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 26:
#line 232 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = build_literal_int_expr((yyvsp[0].literal)); }
#line 1554 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 31:
#line 239 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.expr) = (yyvsp[-1].expr); }
#line 1560 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 32:
#line 246 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_function((yyvsp[-2].item), (yyvsp[0].expr)); }
#line 1566 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 33:
#line 247 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_function((yyvsp[-2].item), (yyvsp[0].expr)); }
#line 1572 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 34:
#line 250 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_function_profile((yyvsp[-4].identifier), (yyvsp[-2].items), (yyvsp[0].item), (yyvsp[-7].i), (yyvsp[-6].i)); }
#line 1578 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 35:
#line 253 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = build_items(); }
#line 1584 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 37:
#line 257 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item(build_items(), (yyvsp[0].item)); }
#line 1590 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 38:
#line 258 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item((yyvsp[-2].items), (yyvsp[0].item)); }
#line 1596 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 39:
#line 261 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_variable((yyvsp[-1].identifier), (yyvsp[0].item), (yyvsp[-2].i), 0); }
#line 1602 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 40:
#line 264 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_module((yyvsp[-3].identifier), (yyvsp[-1].items)); }
#line 1608 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 41:
#line 267 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = build_items(); }
#line 1614 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 43:
#line 271 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item(build_items(), (yyvsp[0].item)); }
#line 1620 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 44:
#line 272 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item((yyvsp[-1].items), (yyvsp[0].item)); }
#line 1626 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 49:
#line 281 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_struct_type((yyvsp[-3].identifier), (yyvsp[-1].items), (yyvsp[-5].i)); }
#line 1632 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 50:
#line 284 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = build_items(); }
#line 1638 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 52:
#line 286 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = (yyvsp[-1].items); }
#line 1644 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 53:
#line 289 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item(build_items(), (yyvsp[0].item)); }
#line 1650 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 54:
#line 290 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.items) = push_item((yyvsp[-2].items), (yyvsp[0].item)); }
#line 1656 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 55:
#line 293 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_variable((yyvsp[-1].identifier), (yyvsp[0].item), 1, (yyvsp[-2].i)); }
#line 1662 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 56:
#line 296 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_unresolved_type((yyvsp[0].identifier)); }
#line 1668 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 57:
#line 297 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_lambda_type((yyvsp[-2].items), (yyvsp[0].item)); }
#line 1674 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 58:
#line 298 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = (yyvsp[-1].item); }
#line 1680 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 59:
#line 299 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.item) = build_ref_type((yyvsp[0].item)); }
#line 1686 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 61:
#line 307 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[0].literal)); }
#line 1692 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 62:
#line 310 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[-1].literal)); }
#line 1698 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 63:
#line 311 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[-1].literal)); }
#line 1704 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 64:
#line 312 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[-1].literal)); }
#line 1710 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 65:
#line 313 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[-1].literal)); }
#line 1716 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 66:
#line 314 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.identifier) = build_ident((yyvsp[-1].literal)); }
#line 1722 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 67:
#line 317 "./grammar/parser.y" /* yacc.c:1646  */
    {(yyval.i) = 0; }
#line 1728 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 68:
#line 318 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.i) = 1; }
#line 1734 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 69:
#line 321 "./grammar/parser.y" /* yacc.c:1646  */
    {(yyval.i) = 0; }
#line 1740 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 70:
#line 322 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.i) = 1; }
#line 1746 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 71:
#line 325 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.i) = 0; }
#line 1752 "./grammar/parser.h" /* yacc.c:1646  */
    break;

  case 72:
#line 326 "./grammar/parser.y" /* yacc.c:1646  */
    { (yyval.i) = 1; }
#line 1758 "./grammar/parser.h" /* yacc.c:1646  */
    break;


#line 1762 "./grammar/parser.h" /* yacc.c:1646  */
      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", yyr1[yyn], &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */

  yyn = yyr1[yyn];

  yystate = yypgoto[yyn - YYNTOKENS] + *yyssp;
  if (0 <= yystate && yystate <= YYLAST && yycheck[yystate] == *yyssp)
    yystate = yytable[yystate];
  else
    yystate = yydefgoto[yyn - YYNTOKENS];

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYEMPTY : YYTRANSLATE (yychar);

  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
#if ! YYERROR_VERBOSE
      yyerror (YY_("syntax error"));
#else
# define YYSYNTAX_ERROR yysyntax_error (&yymsg_alloc, &yymsg, \
                                        yyssp, yytoken)
      {
        char const *yymsgp = YY_("syntax error");
        int yysyntax_error_status;
        yysyntax_error_status = YYSYNTAX_ERROR;
        if (yysyntax_error_status == 0)
          yymsgp = yymsg;
        else if (yysyntax_error_status == 1)
          {
            if (yymsg != yymsgbuf)
              YYSTACK_FREE (yymsg);
            yymsg = (char *) YYSTACK_ALLOC (yymsg_alloc);
            if (!yymsg)
              {
                yymsg = yymsgbuf;
                yymsg_alloc = sizeof yymsgbuf;
                yysyntax_error_status = 2;
              }
            else
              {
                yysyntax_error_status = YYSYNTAX_ERROR;
                yymsgp = yymsg;
              }
          }
        yyerror (yymsgp);
        if (yysyntax_error_status == 2)
          goto yyexhaustedlab;
      }
# undef YYSYNTAX_ERROR
#endif
    }



  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:

  /* Pacify compilers like GCC when the user code never invokes
     YYERROR and the label yyerrorlab therefore never appears in user
     code.  */
  if (/*CONSTCOND*/ 0)
     goto yyerrorlab;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += YYTERROR;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYTERROR)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  yystos[yystate], yyvsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", yystos[yyn], yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;

/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;

#if !defined yyoverflow || YYERROR_VERBOSE
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  /* Fall through.  */
#endif

yyreturn:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  yystos[*yyssp], yyvsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif
#if YYERROR_VERBOSE
  if (yymsg != yymsgbuf)
    YYSTACK_FREE (yymsg);
#endif
  return yyresult;
}
