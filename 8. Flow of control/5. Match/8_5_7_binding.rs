fn age() -> u32 {
  12
}

fn some_number() -> Option<u32> {
  Some(3)
}

fn main() {
  println!("Tell me what kind of person you are?");

  match age() {
    0 => println!("I haven't celebrate my first birthday yet"),
    n @ 1 ..= 12 => println!("I am a child of age: {n}"),
    n @ 13 ..= 19 => println!("I am a teen of age: {n}"),
    n => println!("I am an old person of age: {n}"),
  }

  match some_number() {
    Some(n @ 33) => println!("Number {n} is interesting"),
    Some(n) => println!("Number {n} is not interesting"),
    _ => (),
  }
}