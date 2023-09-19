use super::bin::{process_args, HELP_MESSAGE};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

// helper function to convert stdout buffer to string
fn string_from_buffer(buffer: Vec<u8>) -> String {
  buffer.iter().map(|x| *x as char).collect()
}

// test helper function
fn test_cli_output(cli_commands: Vec<&str>, condition: fn(String) -> bool) {
  for command in cli_commands {
    // create cli args
    let cli_args: Vec<String> = command.split_whitespace().map(|x| x.to_string()).collect();
    // create stdout buffer
    let mut stdout = Vec::new();
    // mutates stdout
    process_args(cli_args, &mut stdout);
    let output = string_from_buffer(stdout);
    assert!(condition(output.trim().to_string()), "Test failed for command: {}", command);
  }
}

#[test]
fn version_flag() {
  // --version
  let condition = |output: String| output == format!("random_stuff v{}", env!["CARGO_PKG_VERSION"]);
  test_cli_output(vec![
    "random_stuff --version", 
    "random_stuff -v"
    ], 
    condition
  );
}

#[test]
fn copy_flag() {
  // --copy
  let condition = |output: String| {
    let mut context = ClipboardContext::new().unwrap();
    let split: Vec<&str> = output.split("\n").collect();
    let clipboard = context.get_contents().unwrap();
    clipboard == split[0] && split[1] == "(copied to clipboard)"
  };
  test_cli_output(vec![
    "random_stuff 1000 symbols --copy", 
    "random_stuff symbols -c 1000",
    "random_stuff -c 1000 symbols"
    ], 
    condition
  );
}

#[test]
fn help_flag() {
  // --help
  let condition = |output: String| output == HELP_MESSAGE.trim();
  test_cli_output(vec!["random_stuff --help", "random_stuff -h"], condition);
}

#[test]
fn random_num() {
  // num
  test_cli_output(vec!["random_stuff 10", "random_stuff 10 num", "random_stuff num 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| c.is_ascii_digit())
  });
}

#[test]
fn random_alpha() {
  // alpha
  test_cli_output(vec!["random_stuff 10 alpha", "random_stuff alpha 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
  });
}

#[test]
fn random_alphacaps() {
  // alphacaps
  test_cli_output(vec!["random_stuff 10 alphacaps", "random_stuff alphacaps 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
  });
}

#[test]
fn random_alpha_plus_caps() {
  // alpha+caps
  test_cli_output(vec!["random_stuff 10 alpha+caps", "random_stuff alpha+caps 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_alphabetic()
    })
  });
}

#[test]
fn random_alphanum() {
  // alphanum
  test_cli_output(vec!["random_stuff 10 alphanum", "random_stuff alphanum 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      if c.is_ascii_digit() {
        return true;
      }
      c.is_ascii_alphabetic() && c.is_ascii_lowercase()
    })
  });
}

#[test]
fn alphanumcaps() {
  // alphanumcaps
  test_cli_output(vec!["random_stuff 10 alphanumcaps", "random_stuff alphanumcaps 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_ascii_uppercase())
    })
  });
}

#[test]
fn random_alphanum_plus_caps() {
  // alphanum+caps
  test_cli_output(vec!["random_stuff 10 alphanum+caps", "random_stuff alphanum+caps 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_alphanumeric()
    })
  });
}

#[test]
fn random_hex() {
  // hex
  test_cli_output(vec!["random_stuff 10 hex", "random_stuff hex 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_lowercase())
    })
  });
}

#[test]
fn random_hexcaps() {
  // hexcaps
  test_cli_output(vec!["random_stuff 10 hexcaps", "random_stuff hexcaps 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_uppercase())
    })
  });
}

#[test]
fn random_symbols() {
  // symbols
  test_cli_output(vec!["random_stuff 10 symbols", "random_stuff symbols 10"], |rando| {
    rando.len() == 10 && rando.chars().all(|c| {
      c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
    })
  });
}
