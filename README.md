## pico-args
[![Build Status](https://travis-ci.org/RazrFalcon/pico-args.svg?branch=master)](https://travis-ci.org/RazrFalcon/pico-args)
[![Crates.io](https://img.shields.io/crates/v/pico-args.svg)](https://crates.io/crates/pico-args)
[![Documentation](https://docs.rs/pico-args/badge.svg)](https://docs.rs/pico-args)
[![Rust Stable](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

An ultra simple CLI arguments parser.

- Only flags, options, free arguments and subcommands are supported.
- Arguments can be separated by a space or `=`.
- Non UTF-8 arguments are supported.
- No help generation.
- No combined flags (like `-vvv`, `-abc` or `-j1`).
- Arguments are parsed in a linear order. From first to last.

### Example

```rust
struct Args {
    help: bool,
    version: bool,
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    free: Vec<String>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| "not a number".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    // Arguments can be parsed in any order.
    let args = Args {
        // You can use a slice for multiple commands
        help: args.contains(["-h", "--help"]),
        // or just a string for a single one.
        version: args.contains("-V"),
        // Parses an optional value that implements `FromStr`.
        number: args.opt_value_from_str("--number")?.unwrap_or(5),
        // Parses an optional value that implements `FromStr`.
        opt_number: args.opt_value_from_str("--opt-number")?,
        // Parses an optional value using a specified function.
        width: args.opt_value_from_fn("--width", parse_width)?.unwrap_or(10),
        // Will return all free arguments or an error if any flags are left.
        free: args.free()?,
    };

    Ok(())
}
```

### Alternatives

The core idea of `pico-args` is to provide some "sugar" for arguments parsing without
a lot of overhead (binary or compilation time wise).
There are no point in comparing parsing features since `pico-args` supports
only the bare minimum. So we will compare only the size overhead and compilation time.

There are a lot of arguments parsing implementations, but we will use only these one:

- [clap](https://crates.io/crates/clap) - is the most popular and complete one
- [gumdrop](https://crates.io/crates/gumdrop) - a simple parser that uses procedural macros
- [structopt](https://crates.io/crates/structopt) - a two above combined

|                   | null    | `pico-args` | `clap`   | `gumdrop` | `structopt` | `argh`      |
|-------------------|---------|-------------|----------|-----------|-------------|-------------|
| Binary overhead   | 0KiB    | 18.8KiB     | 390.6KiB | 22.0KiB   | 390.8KiB    | **18.1KiB** |
| Build time        | 0.1s    | **0.5s**    | 5.5s     | 9s        | 17.5s       | 6.4s        |
| Tested version    | -       | 0.3.1       | 2.33.0   | 0.7.0     | 0.3.9       | 0.1.3       |

- Binary size overhead was measured by subtracting the `.text` section size of an app with
  arguments parsing and a hello world app.
- Build time was measured using `hyperfine 'cargo clean; cargo build --release'`.
- Test projects can be found in `test-apps/`.

### License

MIT
