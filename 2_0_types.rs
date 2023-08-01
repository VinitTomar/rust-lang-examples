fn main() {
  let logical: bool = false;
  println!("Boolean variable: {}", logical);

  let a_float: f64 = 1.3;
  let an_int = 5i32;
  println!("A float 64bit: {}\nAn integer 32bit: {}", a_float, an_int);

  let mut mutable = 12;
  println!("Mutable when integer before mutation: {}", mutable);

  mutable = 21;
  println!("Mutable when integer after mutation: {}", mutable);

  // throw error
  // mutable = false;

  // shadowing
  // no error thrown
  let mutable = false;
  println!("Mutable boolean after shadowing: {}", mutable);

}