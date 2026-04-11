# Rust by Example

Rust
- Modern
- Systems(emphasis on the "s") programming language
- Safe
- Fast
- Concurrent
- Memory safe
- No Garbage collection


## Hello World: `hello.rs`

- `println!`: a macro that prints text to the console
	* -Now what's a "macro"?-
- `rustc` is the compiler command to generate a binary from the single source
	```rust
	rustc hello.rs
	```
- The executable will be named after the filename(no `.rs`!) in this case `hello`

**Macros in Rust**
- Powerful
- Metaprogramming
- Names end with a bang(`!`)
- Expanded with the source and compiled
- Expanded to AST - ensures unexpeted precendence bugs
	* What are "precendence bugs"???
- Usefulness
	- DRY ???
	- DSL ???
	- Vargs interfaces ???

**Errors in Rust**
The compilation errors of the rust are something

```err
error[E0765]: unterminated double quote string
  --> comments.rs:15:43
   |
15 |       println!("Is `x` 10 or 100? x = {}", x");
   |  ___________________________________________^
16 | | }
   | |__^

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0765`.
```

now to the coolest part, just try this...

```bash
rustc --explain E0765
```

## Comments in Rust
- Single line, that starts with `//`
- Block, that are within `/*` and `*/`
- Block comments can exists with an expression

## Formatting output in rust
- All are macros
- Exist within `std::fmt`
- `foramt!` write to string
- `print!` and `println!` to `io::stdout` w/o and w/ newline
- `eprint!` and `eprintln!` to `io::stderr` w/o and w/ newline
- Structs have to implement `fmt::Display` to be passed as arguments directly
- `std::fmt` contains mamy `traits` which govern the display/formatting of text
	* What the hell are `traits`???
- `fmt::Debug` uses `{?}` marker. Formatting text for debugging(anything) purposes
- `fmt::Display` uses the `{}` marker. Pretty formatting text

## Debug diplays

- Use defined types i.e. `struct`(s) cannot be formated for output
- `fmt::Display` trait HAS to be implemented printable in any form
- ANY user defined type(`struct`) CAN `derive` `fmt::Debug` trait using the attribute
- To print using the debug mode use `{:?}`, all the primitives of the `{}` apply to this as well
- To pretty print `struct`'s using `println!` use the `{:#?}`
