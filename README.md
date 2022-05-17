# Duck Compiler

## Overview 

A compiler for the esoteric programming language 
[Duck Duck Goose](https://esolangs.org/wiki/Duck_Duck_Goose)
written in Rust. Currently only supports Linux x86-64.
Uses `gcc` to assemble the generated x86-64 code. 

## Usage

Build the project with cargo:

`cargo build --release`

This will produce `./target/release/gdd`.

`gdd` takes a single argument specifying the path to the `.ddg` file.

`./target/release/gdd ./examples/helloworld.ddg`

This will produce a `helloworld` binary in the current directory.

`./helloworld`

Alternatively, `cargo run ./examples/helloworld.ddg` will do the same thing.

## Write Up (Thoughts)

This project was mostly an excuse to practice writing Rust
(also I am not experienced in compiler design).
The actual Duck Duck Goose (DDG) language is fairly easy
to parse. Determining operations and operands simply
involves counting ducks. However, the rotational
aspect of the language makes things slightly more
complicated. 

My first approach to this was
to statically calculate the rotated duck positions
based on the adjustments made to the goose
over the program's instructions
(`parse::_apply_goose_update()`) and dedicate
registers to hold specific duck values. While this would
work for a program with no control flow like `helloworld.ddg`,
it would quickly derail in the presence of loops. Additionally,
by dedicating registers, the number of ducks a program
could utilize was limited by register count.

Adding a layer of indirection and allocating the ducks
on the stack resolved the issues with the first approach,
but doing so does bring about overhead by 
needing to calculate duck indexes for each
operation to load from. Additionally,
most of the written logic for this approach
is architecture specific (x86-64) which limits
portability. 
