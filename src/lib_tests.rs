// imported by lib.rs

use crate::OUTPUT_TYPES;
use crate::VERSION;
use crate::random;

// test helper function
fn test_output (output_type: &str, condition: fn(String) -> bool) {
  {
    for i in 0..100 {
      let rando: String = random(i, output_type).unwrap();
      assert_eq!(rando.len(), i);
      assert!(condition(rando), "Test failed for output_type: {}", output_type);
    }
  }
}

#[test]
fn version_export() {
  let env_version = env!("CARGO_PKG_VERSION");
  assert_eq!(VERSION, env_version);
}

#[test]
fn output_types_export() {
  assert_eq!(OUTPUT_TYPES.len() > 0, true);
  assert_eq!(OUTPUT_TYPES.contains(&"num"), true);
  assert_eq!(OUTPUT_TYPES[..], OUTPUT_TYPES);
}

#[test]
fn random_num() {
  // num
  test_output("num", |rando| rando.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn random_alpha() {
  // alpha
  test_output("alpha", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_alphabetic() && c.is_ascii_lowercase()
    })
  });
}

#[test]
fn random_alphacaps() {
  // alphacaps
  test_output("alphacaps", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_alphabetic() && c.is_ascii_uppercase()
    })
  });
}

#[test]
fn random_alpha_plus_caps() {
  // alpha+caps
  test_output("alpha+caps", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_alphabetic()
    })
  });
}

#[test]
fn random_alphanum() {
  // alphanum
  test_output("alphanum", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_digit() ||(c.is_ascii_alphabetic() && c.is_ascii_lowercase())
    })
  });
}

#[test]
fn random_alphanumcaps() {
  // alphanumcaps
  test_output("alphanumcaps", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_ascii_uppercase()) 
    })
  });
}

#[test]
fn random_alphanum_plus_caps() {
  // alphanum+caps
  test_output("alphanum+caps", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_alphanumeric()
    })
  });
}

#[test]
fn random_hex() {
  // hex
  test_output("hex", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_lowercase())
    })
  });
}

#[test]
fn random_hexcaps() {
  // hexcaps
  test_output("hexcaps", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_digit() || (c.is_ascii_hexdigit() && c.is_ascii_uppercase())
    })
  });
}

#[test]
fn random_symbols () {
  // symbols
  test_output("symbols", |rando| {
    rando.chars().all(|c| {
      c.is_ascii_punctuation() || c.is_ascii_alphanumeric()
    })
  });
}
