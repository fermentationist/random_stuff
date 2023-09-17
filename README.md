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
        - `num` - numbers only (default)
        - `alpha` - lowercase letters only 
        - `alphawcaps` - lowercase and uppercase letters
        - `alphanum` - numbers and lowercase letters 
        - `alphanumwcaps` - numbers with lowercase and uppercase letters
    If no output type is specified, the default value of `num` will be used. 

 To copy the output to your clipboard, add the `--copy` or `-c` flag to the command. For example, `random_stuff 10 alphanum -c` will generate a random string of 10 numbers and letters, including capital and lowercase letters, and copy it to your clipboard.

```JavaScript
// Generate a random string of 10 numbers
random_stuff 10

// Generate a random string of 10 numbers and letters, including capital and lowercase letters
random_stuff 10 alphanumwcaps

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

2. **`OUTPUT_TYPES`** - *`[&str; 5]`*
An array of allowed output types - it can be imported with `use random_stuff::OUTPUT_TYPES;`.

```rust
use random_stuff::{random, OUTPUT_TYPES};

main () {
    println!("OUTPUT_TYPES: {:?}", OUTPUT_TYPES);
    // OUTPUT_TYPES: ["num", "alpha", "alphawcaps", "alphanum", "alphanumwcaps"]
    let output = random(10, "alphanumwcaps");
    match output {
        Ok(s) => println!("Output: {}", s),
        // Output: OYipyfgGZL
        Err(e) => println!("Error: {}", e),
    }
}
```

---

### License

#### Copyright © 2023 [Dennis Hodges](https://dennis-hodges.com)

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

Source: http://opensource.org/licenses/ISC