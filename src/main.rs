use std::env;
use random_stuff;

fn main() {
    // get command-line args
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: Missing argument");
        return;
    }

    let mut copy_flag = false;
    // check args for --copy flag and remove from args if found
    if args.contains(&String::from("--copy")) || args.contains(&String::from("-c")) {
        let options = ["-c", "--copy"];
        args.retain(|arg| !options.contains(&arg.as_str()));
        copy_flag = true;
    }

    let mut output_type = "num";
    // check args for output type and remove from args when found
    for o_type in random_stuff::OUTPUT_TYPES {
        if args.contains(&String::from(o_type)) {
            output_type = o_type;
            args.retain(|arg| arg != o_type);
        }
    }

    let output_length: usize;
    // check args for output length
    match &args[1].parse::<usize>(){
        Ok(num) => output_length = *num,
        Err(_) => {
            println!("Error: Invalid argument");
            return;
        }
    };
    // use lib to get random result
    let result = random_stuff::random(output_length, output_type); 
    match result {
        Ok(output) => {
            // print output to terminal
            println!("{}", output);
            // if copy_flag is true, copy output to clipboard
            if copy_flag {
                use cli_clipboard::{ClipboardContext, ClipboardProvider};
                let mut context = ClipboardContext::new().unwrap();
                let _ = context.set_contents(output);
                println!("(copied to clipboard)")
            }
        },
        Err(message) => println!("Error: {}", message)
    }
}
