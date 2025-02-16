This is the [Rust](https://rust-lang.org)/[Cargo](https://crates.io) implementation of Rusl.

- I am aware that there is an existing [rusl package](https://crates.io/crates/rusl) on cargo. This is not the rusl package from here. Because of this I have renamed the rust version to `rsrusl`.

You can see the [crates.io/cargo package here](https://crates.io/crates/rsrusl).

> [!IMPORTANT]
> I am aware that there is an existing [rusl package](https://crates.io/crates/rusl) on cargo. This is not the rusl package from here. Because of this I have renamed the rust version to `rsrusl`.

## How to use:
1. Make a Rust project with [Cargo](https://crates.io) with: `cargo new projectName` and `cd projectName`.
2. Add rusl to the project using: `cargo add rsrusl`
3. Use rusl in your project like:
```rs
use rsrusl;

fn main() {
    // Use our rusl library functions

    rsrusl::hw();
    let input = rsrusl::i::userinput("Enter your name: ");
    rsrusl::g(&input);
}
```
