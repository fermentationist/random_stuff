use rand::Rng;
use std::result::Result::Err;

// import unit_tests
#[cfg(test)]
mod lib_tests;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const OUTPUT_TYPES: [&str; 10] = ["num", "alpha", "alphacaps", "alpha+caps", "alphanum", "alphanumcaps", "alphanum+caps", "hex", "hexcaps", "symbols"];

pub fn random(length: usize, output_type: &str) -> Result<String, String> {
  if !OUTPUT_TYPES.contains(&output_type) {
    return Err(format!("Invalid output_type - {}", output_type));
  }
  const NUM: &str = "0123456789";
  const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
  let hex = NUM.to_string() + &ALPHA[..6];
  let caps: String = ALPHA[..].to_uppercase().chars().collect();
  let hexcaps = NUM.to_string() + &caps[..6];
  let mut range = rand::thread_rng();
  let alpha_w_caps = ALPHA.to_string() + &caps;
  let alphanum = ALPHA.to_string() + NUM;
  let alphanumcaps = caps.clone() + NUM;
  let alphanum_w_caps = ALPHA.to_string() + &caps + NUM;
  let symbols = alphanum_w_caps.clone() + "!@#$%^&*-_=+;:'\",<.>/?";
  let chars = match output_type {
    "num" => NUM,
    "alpha" => ALPHA,
    "alphacaps" => &caps,
    "alpha+caps" => &alpha_w_caps,
    "alphanum" => &alphanum,
    "alphanumcaps" => &alphanumcaps,
    "alphanum+caps" => &alphanum_w_caps,
    "hex" => &hex,
    "hexcaps" => &hexcaps,
    "symbols" => &symbols,
    &_ => &alphanum_w_caps
  };
  let output = (0..length).map(|_| {
    let random_index = range.gen_range(0..chars.len());
    chars.chars().nth(random_index).unwrap()
  }).collect::<String>();
  Ok(output)
}