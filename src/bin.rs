use random_stuff;
use std::io::Write;

// import unit_tests
// #[cfg(test)]
// use super::bin_tests;
// mod bin_tests;

pub const HELP_MESSAGE: &str = "\nUsage: \nrandom_stuff <length> [output_type] [copy_flag]\n\nlength (required): the length of the random string to generate (must be a positive integer)\noutput_type (optional): the type of random string to generate\ncopy_flag (optional): add --copy or -c to copy the random string to the clipboard\n\nAllowed output_type values:\n  num: numbers only\n  alpha: lowercase letters only\n  alphacaps: uppercase letters only\n  alpha+caps: lowercase and uppercase letters only\n  alphanum: lowercase letters and numbers only\n  alphanumcaps: uppercase letters and numbers only\n  alphanum+caps: lowercase and uppercase letters and numbers only\n  hex: hexadecimal numbers only\n  hexcaps: hexadecimal numbers only (uppercase)\n  symbols: lowercase and uppercase letters and numbers and symbols\nDefault: num\n";

fn print_help(mut writer: impl Write) {
  writeln!(writer, "{}", HELP_MESSAGE).unwrap();
}

fn print_version(mut writer: impl Write) {
  writeln!(writer,"random_stuff v{}", random_stuff::VERSION).unwrap();
}

pub fn process_args(mut args: Vec<String>, mut writer: impl Write) {
  if args.len() < 2 {
      writeln!(writer, "Error: Missing argument").unwrap();
      return;
  }

  // check args for --version flag and print version if found
  if args.contains(&String::from("--version")) || args.contains(&String::from("-v")) {
      print_version(writer);
      return;
  }

  // check args for --help flag and print help text if found
  if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
      print_help(writer);
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
          writeln!(writer,"Error: Invalid argument").unwrap();
          return;
      }
  };
  // use lib to get random result
  let result = random_stuff::random(output_length, output_type); 
  match result {
      Ok(output) => {
          // print output to terminal
          writeln!(writer,"{}", output).unwrap();
          // if copy_flag is true, copy output to clipboard
          if copy_flag {
              use cli_clipboard::{ClipboardContext, ClipboardProvider};
              let mut context = ClipboardContext::new().unwrap();
              let _ = context.set_contents(output);
              writeln!(writer,"(copied to clipboard)").unwrap()
          }
      },
      Err(message) => writeln!(writer,"Error: {}", message).unwrap()
  }
}