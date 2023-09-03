# Mephisto Compiler

This is a compiler for the Mephisto programming language. It is written in Rust.

## Building

To build the compiler, run `cargo build` in the root directory of the project.

## Running

TBD

## Testing

Run `cargo test` in the directory of the project.

## Language Features

## Planned Features and TODOs

* [ ] Fix the AR envelope, or create a new retriggerable one
* [ ] Create The Instrument
* [ ] Rewrite the code generation to create a proper modular system with an ability to create and reconnect modules
* [ ] Fix parsing of function calls (now you can't just call a function without assigning the result to a variable)
* [ ] Use a proper lexer and parser generator instead of a handwritten lexer and parser
* [ ] Fix import system (now the path resolution is broken)
* [ ] Add support for "if" expressions
* [ ] Add support for enums
* [ ] Create a WebAssembly backend
* [ ] Create Rust backend
* [ ] Create optimizing passes (at least constant folding and friends)
* [ ] Create AU and VST backends
* [x] Fix tokenizing comments — now at least in imports and in connect block comments are broken. Technically, can be fixed with the next item
