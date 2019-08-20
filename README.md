# typed-css

## what?

A rust crate to write css in rust.

## why?

To avoid writing invalid css.


## how?

    # using cargo-edit
    cargo add --git https://github.com/docteurklein/typed-css


```rust
use typed_css::css;

fn main() {
    println!("{}", css! {
        color: blue;
    });
}
```
