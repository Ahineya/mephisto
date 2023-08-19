
+-----------------------------+--------------------------------------------------------------------------------------+
|         Compiler Phase      |                                        Description                                   |
+-----------------------------+--------------------------------------------------------------------------------------+
| 1. Lexical Analysis         | Converts the source code into a stream of tokens.                                     |
|                             |                                                                                      |
| 2. Syntax Analysis (Parsing)| Converts the token stream into a parse tree or abstract syntax tree (AST).            |
|                             |                                                                                      |
| 3. Semantic Analysis        | **<- Your check (Definite Assignment Analysis) goes here.**                           |
|                             | Checks the AST for semantic correctness. This phase ensures that the program has      |
|                             | valid meanings by checking types, variable bindings, and other language-specific      |
|                             | constraints.                                                                         |
|                             |                                                                                      |
| 4. Intermediate Code Gen    | Transforms the AST into an intermediate code representation, often for optimization.  |
|                             |                                                                                      |
| 5. Code Optimization        | Improves the intermediate code for better performance, but retains the same meaning.  |
|                             |                                                                                      |
| 6. Code Generation          | Converts the optimized intermediate code into target machine code or bytecode.        |
|                             |                                                                                      |
| 7. Symbol Table Management  | Manages a table of symbols used in the source code, often used throughout many phases.|
+-----------------------------+--------------------------------------------------------------------------------------+


+----------------------------------+--------------------------------------------------------------------------------------+
|        Semantic Substep          |                                        Description                                   |
+----------------------------------+--------------------------------------------------------------------------------------+
| 1. Name Binding and Resolution   | Associates names with their intended entities (like variables, types, functions).     |
|                                  | Resolves ambiguities in names, especially in languages that support namespaces,      |
|                                  | modules, or scopes.                                                                  |
|                                  |                                                                                      |
| 2. Type Checking                 | Ensures that operations and functions are used with the correct type of data.         |
|                                  | Infers types where not explicitly provided.                                          |
|                                  |                                                                                      |
| 3. Definite Assignment Analysis  | **Ensures that variables are initialized before they are used.**                      |
|                                  |                                                                                      |
| 4. Control Flow Analysis         | Analyzes the possible paths through the program to support other checks.              |
|                                  |                                                                                      |
| 5. Reachability Analysis         | Checks that every statement in the program can be executed (e.g., there's no code     |
|                                  | after a return statement that would never be reached).                               |
|                                  |                                                                                      |
| 6. Exception Handling Analysis   | Ensures that exceptions are properly caught or declared to be thrown.                |
|                                  |                                                                                      |
| 7. Uniqueness and Ownership      | For languages that have unique ownership models (like Rust), ensures that data        |
|    Analysis                      | ownership and borrowing rules are adhered to.                                        |
|                                  |                                                                                      |
| 8. Overload Resolution           | Determines the correct version of a function or operator to call when multiple        |
|                                  | versions are possible (common in languages that support function/operator overloading|
|                                  | or generics).                                                                        |
|                                  |                                                                                      |
| 9. Constant Folding              | Evaluates constant expressions at compile time to optimize them.                      |
|                                  |                                                                                      |
| 10. Limit Checks                 | Ensures that certain language-specific limits are adhered to, like maximum array      |
|                                  | sizes or recursion depths.                                                           |
+----------------------------------+--------------------------------------------------------------------------------------+
