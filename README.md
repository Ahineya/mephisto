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

* [ ] Add unary ! operator (technically not mandatory (1 - n is the same as !n in Mephisto), but it's a nice feature to have)
* [ ] Fix sequencer BPM control
* [ ] Create The Instrument
* [ ] Use a proper lexer and parser generator instead of a handwritten lexer and parser
* [ ] Fix import system (now the path resolution is broken)
* [ ] Add support for "if" expressions
* [ ] Closures (probably). Can be very useful for some algorithms, for example, the smoothing algorithm. Now it's implemented as a module
* [ ] Add support for enums?
* [ ] Create optimizing passes (at least constant folding and friends)
* [ ] Create a WebAssembly backend
* [ ] Create Rust backend
* [ ] Create AU and VST backends
* [ ] Include params into the audio graph generation
* [ ] Add an ability to create modules on the fly
* [x] Add an ability to reconnect modules in runtime
* [x] Fix parsing of function calls (now you can't just call a function without assigning the result to a variable)
* [x] Fix tokenizing comments — now at least in imports and in connect block comments are broken. Technically, can be fixed with the next item
