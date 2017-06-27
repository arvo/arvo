# Intermediate Representations

Intermediate representations (IRs) are abstract data types that are used to represent a program at different stages of its compilation.

## Abstract Syntax Tree

The abstract syntax tree (AST) is the representation of the raw input, after it has been tokenised and parsed. The tokenisation process validates individual keywords and elements, and the parser checks that they are assembled in a way that is grammatically correct. At this stage, no semantic analysis has been done to ensure that the program is actually valid.

## Abstract Intermediate Representation

The abstract intermediate representation (AIR) is resolved from an AST. Resolution checks that symbols are brought into scope before they are used, and are not used after they exit scope. At this stage, types have not necessarily been resolved.

## Normalised Intermediate Representation

The normalised intermediate representation (NoIR) is the normalised form of the AIR. NoIR is produced by normalising an AIR that has been through all necessary compilation passes, and so by the time a program has reached this stage of compilation, it is valid. Optimisation passes, and code generation passes, are done using NoIR. 