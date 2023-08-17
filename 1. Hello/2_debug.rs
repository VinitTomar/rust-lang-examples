#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
  
  // same as {}
  println!("{:?}", 12);
  println!("{1:?}, {0:?}, {actor:?}", "ab", "bc", actor="some");

  println!("Structure => {:?}", Structure(3));
  println!("Deep => {:?}", Deep(Structure(7)));

  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  // Pretty print
  println!("{:#?}", peter);
  
}

