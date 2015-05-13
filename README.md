# Intro to Rust for Web Developers

This introductory workshop aims to introduce the Rust programming language to
web developers.

## Requirements

You need to have both Ruby and Rust installed, the
[recommended](https://doc.rust-lang.org/book/installing-rust.html) way to
install Rust is to use this script:

```
$ curl -sf -L https://static.rust-lang.org/rustup.sh | sh
```

This script should install the latest stable version of Rust and also the
[Cargo](https://crates.io/) dependency manager for Rust.

The Ruby version used to write the source code was 2.2.1, but it doesn’t use
any 2.x specific syntax so it could theoretically work with 1.9.x and 1.8.x but
this isn’t tested!

### Sanity Check

To check if the library is in working order, run the following commands:

```
$ cargo run
```

This should compile the simple “Hello World!” Rust example. To test the Ruby
side of things, run the following command:

```
$ ruby ruby/code_breaker.rb
```

This should run the tests for the Ruby code base, all tests should be passing.

## Task

The format is simple, reimplement an existing code base written in Ruby to a
Rust program. The existing code base can be found in:

* `ruby` - The code written in Ruby

The problems have been inspired by the
[the matasano crypto challenges](http://cryptopals.com/), in particular problem
[5](http://cryptopals.com/sets/1/challenges/5/) and a simpler version of
[6](http://cryptopals.com/sets/1/challenges/6/) from
[set 1](http://cryptopals.com/sets/1/).

## License

This software is distributed under the [MIT](/LICENSE) license.
