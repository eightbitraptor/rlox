# Rlox

Rlox is the tree walking interpreter for the programming language Lox,
fromt Part 1 of [Robert Nystrom's Crafting Interpreters
book](https://craftinginterpreters.com/) book, written in Rust.

## Building

Nothing particularly odd. the standard `cargo build` should sort you
out. You'll need the `nightly` toolchain.

## testing

unit tests are run using `cargo test` as usual. to run rlox against Bob's canonical Lox test suite follow [the instructions here](https://github.com/munificent/craftinginterpreters#testing-your-implementation).

The summary is: [Go here and install Dart](https://dart.dev/get-dart), and then

```
git clone git://github.com/munificent/craftinginterpreters && cd craftinginterpreters
make get
dart tool/bin/test.dart chap04_scanning --interpreter ../rlox/target/debug/rlox
```
