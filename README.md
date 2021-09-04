# For when you just need colors

`const_colors` is a set of macros that allow you to have colors in your Linux terminal without the need to allocate anything to do so.

## How to use

To use, simply use the `concat!` macro in your strings like so

```rust
concat!(red!(), "I'm red, make sure to end me", end!(), green!(), "Now I'm green", end!(), bold!(), blue!(), "Now I am bold blue", end!());
```

To use colors with non-constant values, simply use the `format!` macro in tandom with the `concat!` macro like so

```rust
format!(concat!("Server initialization ", bold!(), "{}", end!()));
println!(concat!("This also works with ", bold!(), "{}", end!(), "formatted data"));
```
