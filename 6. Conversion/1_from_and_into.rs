use std::convert::From;

#[allow(dead_code)]
#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}


fn main() {
  let num = Number::from(44);
  println!("From trait: {:?}", num);

  let integer = 32;
  let num2: Number = integer.into();
  println!("Into trait: {:?}", num2);
}