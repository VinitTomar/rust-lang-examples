#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321_f32;

  // Error! No implicit conversion
  // let integer: u8 = decimal;

  let integer: u8 = decimal as u8;
  let character: char = integer as char;

  // decimal cannot be converted to char directly
  // let character: char = decimal as char;

  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  println!("1000 as a u8 is : {}", 1000 as u8);
  println!(" 232 as a i8 is : {}", 232 as i8);

  println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
  println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
  println!("   nan as u8 is : {}", f32::NAN as u8);

  // This behavior incurs a small runtime cost and can be avoided
  // with unsafe methods, however the results might overflow and
  // return **unsound values**. Use these methods wisely:
  unsafe {
    println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
    println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
    println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
  }

}