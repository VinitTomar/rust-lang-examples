fn main() {
  println!("1+2 = {}", 1i32 + 2);
  println!("1-2 = {}", 1i32 - 2);
  
  // Error "attempt to compute `1_u32 - 2_u32`, which would overflow"
  // println!("1-2 = {}", 1u32 - 2);
}