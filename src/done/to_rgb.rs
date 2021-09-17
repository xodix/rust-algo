pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

use std::str;
use std::u8;

pub fn convert(hex_color: &str) -> Color {
  let r = hex_color[1..3].to_string();
  let g = hex_color[3..5].to_string();
  let b = hex_color[5..7].to_string();
  Color {
    r: u8::from_str_radix(&r, 16).unwrap(),
    g: u8::from_str_radix(&g, 16).unwrap(),
    b: u8::from_str_radix(&b, 16).unwrap(),
  }
}
