# Duck Compiler

## Description

A compiler for the esoteric programming language 
[Duck Duck Goose](https://esolangs.org/wiki/Duck_Duck_Goose)
written in Rust. Currently only supports Linux x86-64.

## Usage

Build the project with cargo:

`cargo build --release`

This will produce `./target/release/gdd`.

`gdd` takes a single argument specifying the path to the `.ddg` file.

`./target/release/gdd ./examples/helloworld.ddg`

This will produce a `helloworld` binary in the current directory.

`./helloworld`