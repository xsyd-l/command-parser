# clio

`clio` is a small Rust library for parsing command-line arguments into
subcommands, options, positional values, and option values.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
clio = { git = "https://github.com/xsyd-l/clio.git" }
```

Pass the process arguments (including the executable name) to `Args::from`.
List the short options that take a value in `config_options`:

```rust
use clio::Args;

fn main() {
    let args = std::env::args().collect();

    let Some(parsed) = Args::from(args, &["-o"]) else {
        eprintln!("Usage: my-app [SUBCOMMAND] [OPTIONS] [ARGS]");
        return;
    };

    println!("Subcommand: {:?}", parsed.subcommand);
    println!("Long options: {:?}", parsed.long_options);
    println!("Short options: {:?}", parsed.short_options);
    println!("Arguments: {:?}", parsed.configs);
    println!("Option values: {:?}", parsed.config_options);
}
```

For example, running `my-app build -o release --verbose input.txt` yields
`build` as the subcommand, `--verbose` as a long option, `input.txt` as a
positional argument, and `release` as the value for `-o`.

## Parsed fields

`Args::from` returns `None` when the argument vector contains only the
executable name. Otherwise, it returns an `Args` with:

- `subcommand`: the first argument after the executable, if it does not start
  with `-`.
- `long_options`: arguments beginning with `--`.
- `short_options`: arguments beginning with `-` but not `--`, excluding
  configured options whose values were consumed.
- `configs`: positional arguments.
- `config_options`: values consumed for the option names supplied to
  `Args::from`.

## Development

Build the crate with:

```sh
cargo build
```

Run its tests with:

```sh
cargo test
```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
