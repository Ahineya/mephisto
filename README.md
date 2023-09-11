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

* [ ] Create The Instrument
* [ ] Use a proper lexer and parser generator instead of a handwritten lexer and parser
* [ ] Closures (probably). Can be very useful for some algorithms, for example, the smoothing algorithm. Now it's implemented as a module
* [ ] Create a WebAssembly backend
* [ ] Create optimizing passes (at least constant folding and friends) (Perhaps should be done after the LLVM or Binaryen backend)
* [ ] Create Rust backend
* [ ] Create AU and VST backends. Perhaps just JUCE backend? Or maybe just a library that can be used in JUCE?
* [ ] Include params into the audio graph generation
* [ ] Add an ability to create modules on the fly?
* [ ] Add !, &&, || operators. Technically not mandatory (1 - n is the same as !n, + is the same as ||, and * is the same as && in Mephisto)
* [ ] Fix import system (now some files are imported twice)
* [x] Fix import system (now the path resolution is broken)
* [x] Add support for "if" statements
* [x] Add an ability to reconnect modules in runtime
* [x] Fix parsing of function calls (now you can't just call a function without assigning the result to a variable)
* [x] Fix tokenizing comments — now at least in imports and in connect block comments are broken. Technically, can be fixed with the next item
