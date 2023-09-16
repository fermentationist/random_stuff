use rand::Rng;
use std::result::Result::Err;

pub const OUTPUT_TYPES: [&str; 5] = ["num", "alpha", "alphawcaps", "alphanum", "alphanumwcaps"];

pub fn random(length: usize, output_type: &str) -> Result<String, String> {
  if !OUTPUT_TYPES.contains(&output_type) {
    return Err(format!("Invalid output_type - {}", output_type));
  }
  const NUM: &str = "0123456789";
  const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
  let caps: String = ALPHA[..].to_uppercase().chars().collect();
  let mut range = rand::thread_rng();
  let alphawcaps = ALPHA.to_string() + &caps;
  let alphanum = &(ALPHA.to_string() + NUM);
  let alphanumwcaps = &(ALPHA.to_string() + &caps + NUM);
  let chars = match output_type {
    "num" => NUM,
    "alpha" => ALPHA,
    "alphawcaps" => alphawcaps.as_str(),
    "alphanum" => alphanum,
    "alphanumwcaps" => alphanumwcaps,
    &_ => alphanumwcaps
  };
  let output = (0..length).map(|_| {
    let random_index = range.gen_range(0..chars.len());
    chars.chars().nth(random_index).unwrap()
  }).collect::<String>();
  Ok(output)
}