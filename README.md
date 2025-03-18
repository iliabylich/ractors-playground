# Ruby, Ractors, and Concurrent Data Structures

This repository contains code for [this article](https://iliabylich.github.io/ruby-ractors-and-concurrent-data-structures/)

Project structures:

1. `rust-atomigs/` - contains Rust code, core of the logic
2. `c_atomics/` - C extension, wraps Rust code
3. `tests/` - contains example of the code

## Building and running

Make sure to have Rust and C compilers installed, also you need a tool called [`just`](https://github.com/casey/just).

First, make sure to install Ruby dependencies by running `bundle`.

1. `just build-atomics` compiles Rust and C code into a single shared object
2. `just build-compile-commands-json` builds `compile_commands.json` for your LSP (like clangd)
3. `just mpmc-queue-simulation` builds and runs the simulation from the section `Better Queue` -> `Marking`
4. `ruby tests/<file>.rb` runs individual example

## Notable examples to try

1. `ruby tests/parallel-tests.rb`
1. `ruby tests/web-server.rb`
