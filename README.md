# random_stuff

## Description
A simple command line tool to generate random strings of numbers and/or letters, written in Rust.

## Usage - Binary executable
You will need to have Rust installed on your system. If you do not have Rust installed, you can find instructions [here](https://www.rust-lang.org/tools/install).

1. Download the repository: `git clone https://github.com/fermentationist/random_stuff.git`.
2. Change into the directory: `cd random_stuff`.
3. To run, you may start the program in one of three ways:
    - Run the program with `cargo run`.
    - Build the program with `cargo build --release`, and then run the binary - `./target/release/random_stuff`. This binary can also be moved and run from anywhere on your system, or even copied to another system (without Rust) and run there.
    - Install the program with `cargo install --path .` and then run with the command `random_stuff` from anywhere.
4. To use, invoke the program with command line arguments specifying the length of the string to generate (an integer, required), and the type of characters to use (optional), which must be one of the following values: 
    - `num` - numbers only [0-9] *default* (10 possible characters, example: `08362261`)
    - `alpha` - lowercase letters only [a-z] (26 possible characters, example: `jxqz`)
    - `alphacaps` - uppercase letters only [A-Z] (26 possible characters, example: `JXQZ`)
    - `alpha+caps` - lowercase and uppercase letters [a-zA-Z] (52 possible characters, example: `jXqZ`)
    - `alphanum` - numbers and lowercase letters  [0-9a-z] (36 possible characters, example: `0x3q2z`)
    - `alphanumcaps` - numbers and uppercase letters  [0-9A-Z] (36 possible characters, example: `0X3Q2Z`)
    - `alphanum+caps` - numbers with lowercase and uppercase letters [0-9a-zA-Z] (62 possible characters, example: `0x3Q2z`)
    - `hex` - hexadecimal numbers [0-9a-f] (16 possible characters, example: `0x3q2z`)
    - `hexcaps` - hexadecimal numbers [0-9A-F] (16 possible characters, example: `0X3Q2Z`)
    - `symbols` - numbers, lowercase and uppercase letters, and symbols [0-9a-zA-Z!@#$%^&*()-_=+[{]}\|;:'",<.>/?] (94 possible characters, example: `0x3Q2z!`)

If no output type is specified, the default value of `num` will be used. 

To copy the output to your clipboard, add the `--copy` or `-c` flag to the command. For example, `random_stuff 10 alphanum+caps -c` will generate a random string of 10 numbers and letters (uppercase and lowercase), and copy it to your clipboard.

```JavaScript
// Generate a random string of 10 numbers
random_stuff 10

// Generate a random string of 10 numbers and letters, including capital and lowercase letters
random_stuff 10 alphanum+caps

// Generate a random string of 10 numbers and letters, and copy it to the clipboard
random_stuff 10 alphanum -c

// Arguments can be in any order
random_stuff alphanum --copy 10
```

## Usage - Library

To use as a library, add the following to your `Cargo.toml` file:
```toml 
[dependencies]
random_stuff = { git = "https://github.com/fermentationist/random_stuff.git" }
```

Then, in your Rust code, import it with `use random_stuff;`. The library has two exports:

1. **`random`** - *`fn random(output_length: usize, output_type: &str) -> Result<String, String>`*
`random` is a function that takes two arguments: the length of the string to generate, and the type of string to generate. It returns a `Result` type, which can be either `Ok(String)` or `Err(String)`. The `Err` type will be returned if the output type is not one of the allowed types listed above. 

2. **`OUTPUT_TYPES`** - *`[&str; 10]`*
An array of allowed output types - it can be imported with `use random_stuff::OUTPUT_TYPES;`.

```rust
use random_stuff::{random, OUTPUT_TYPES};

main () {
    println!("OUTPUT_TYPES: {:?}", OUTPUT_TYPES);
    // OUTPUT_TYPES: ["num", "alpha", "alphacaps", "alpha+caps", "alphanum", "alphanumcaps", "alphanum+caps", "hex", "hexcaps", "symbols"]
    let output = random(10, "alphanum+caps");
    match output {
        Ok(s) => println!("Output: {}", s),
        // Output: OYipyfgGZL
        Err(e) => println!("Error: {}", e),
    }
}
```

## Development

### Testing

To run the tests, use `cargo test`. 

This will run the tests in `lib_tests.rs`, which are the unit tests for the library, as well as `bin_tests.rs`, which act as the "integration" tests for the binary executable. 

In actuality, they are unit tests for the functions invoked by the binary executable to process the command-line arguments. These functions are decoupled from the `main` function that reads the command-line arguments for the sake of testability. This leaves the binary executable itself untested, but it is so simple that a simple manual test is sufficient.

For example, this is `main.rs`, in its entirety (as of this writing):

```rust
use std::env;

mod bin;

#[cfg(test)]
mod bin_tests;

fn main() {
    // get command-line args
    let args: Vec<String> = env::args().collect();
    // pass args to bin::process_args, with stdout as writer
    bin::process_args(args, std::io::stdout());
}
```

This decoupling allows us to only test the `process_args` function, which is the only function that does anything other than read the command-line arguments. The `process_args` function is tested in `bin_tests.rs`, which is run by `cargo test`. 

It could be argued that the "integration" tests indirectly test the library functions, as they are invoked by the `process_args` function, and so the unit tests in `lib_tests.rs` are therefore superfluous. However, I think that unless the time it takes to run them becomes burdenous, it doesn't hurt to have the library functions tested directly, as well as indirectly, as it makes it easier to see what is being tested, and what is not. Besides, I wrote them first, and I'm not going to delete them now ;).

### Dependencies

- [cli-clipboard](https://crates.io/crates/cli-clipboard) - A simple cross-platform clipboard library for Rust.

- [rand](https://crates.io/crates/rand) - A Rust library for random number generation.

---

### License

#### Copyright © 2023 [Dennis Hodges](https://dennis-hodges.com)

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

Source: http://opensource.org/licenses/ISC