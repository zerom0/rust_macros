> Use the term **syntax extension** to avoid conflicts with the keyword `macro`.

Macro expansion happens _after_ the construction of the AST, but before the compiler begins constructing its semantic understanding of the program.

- Syntax extension can _only_ expand to the kind of AST node the parser _expects_ at that position.
- As a consequence of the above, syntax extension _absolutely cannot_ expand to incomplete or syntactically invalid constructs.
- Expansion happens in "passes"; as many as is needed to completely expand all invocations.
    - There is a recursion limit with a default of 128.
    - Crate wide attribute `#![recursion_limit="…"]`
# Built in macros

# Declarative macros

```sh
cargo run --example declarative 
```

# Procedural macros
## Function like

```sh
cargo run --example procedural 
```

## Derive

```sh
cargo run --example derive 
```

## Attributes


# Links
- [The little book of macros](https://veykril.github.io/tlborm/introduction.html)
- [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)
- https://github.com/dtolnay/proc-macro-workshop