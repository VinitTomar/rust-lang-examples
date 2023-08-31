#[allow(dead_code)]

enum Color {
  Red,
  Green,
  Blue,
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}

fn main() {
  let color = Color::CMYK(44, 22, 55, 1);

  match color {
    Color::Red => println!("Color is red"),
    Color::Green => println!("Color is green"),
    Color::Blue => println!("Color is blue"),
    Color::RGB(r, g, b) => println!("Red: {r}, Green: {g}, Blue: {b}"),
    Color::HSV(h, s, v) => println!("Hue: {h}, Saturation: {s}, Value: {v}"),
    Color::HSL(h, s, l) => println!("Hue: {h}, Saturation: {s}, Light: {l}"),
    Color::CMY(c, m , y) => println!("Cyan: {c}, Magenta: {m}, Yellow: {y}"),
    Color::CMYK(c, m , y, k) => println!("Cyan: {c}, Magenta: {m}, Yellow: {y}, key (black): {k}!"),
  }
}