# TODO

## LANGUAGE/INTERPRETER FEATURES:

- [ ] error report with column interval;
- [ ] error report with code snippet;
- [ ] pretty print for error report (rust-like);
- [ ] support for C-like comma operator;
- [ ] support for C-like ternary operator;
- [ ] forbid variable redeclaration (or maybe only allow it at the REPL level);
- [X] allow for escapes sequences inside string literals;
- [ ] allow blocks inside REPL
- [ ] deprecate 'statement-like' "print" and implement native "print" function
- [X] support for lambda functions
- [ ] support for (both prefix and postfix) '++' and '--' operators;

## CODE IMPROVEMENTS/REFACTORS:
- [X] move out of the "index-based string slicing" for string lookups;
- [ ] dependency injection for output 
    - [ ] unit testing based on dependency injection
- [ ] separate control flow exceptions from runtime errors
- [ ] generalize 'unreachability' guard for 'Stmt::Function' inside 'UserDefinedFunction' logic

## BUG FIXES:
- [ ] buggy execution for inline blocks in REPL