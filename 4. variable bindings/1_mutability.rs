fn main() {
  let _immutable_binding = 44;
  let mut mutable_binding = 2;

  println!("Before mutation: {}", mutable_binding);

  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);

  // _immutable_binding += 1;
  // ^ ERROR
}