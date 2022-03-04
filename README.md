# Symbolic Execution ala Rosette written in Rust

The goal is to be able to implement an interpreter for a DSL in Rust without having
to worry about it being symbolic. With extremely minimal changes (hopefully just an annotation on the function),
this library should allow you to lift the concrete interpreter to a symbolic interpreter 
that lets your perform synthesis and verification.

See [Rosette][rosette] for more information on the idea.

## Why Rust?
One, for fun. Two, because Rust is a less esoteric language than Racket. If this works well, it may bring this
technology to a wider audience.

[rosette]: https://emina.github.io/rosette/
