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

The Ruby version used to write the source code was 2.2.1, but it doesn’t use
any 2.x specific syntax so it could theoretically work with 1.9.x and 1.8.x but
this isn’t tested!

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
