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
