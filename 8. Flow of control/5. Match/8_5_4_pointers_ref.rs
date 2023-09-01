fn main() {
  let reference = &4;

  match reference {
    &val => println!("Got a value via destructuring {}", val),
  }

  match *reference {
    val => println!("Got a value via derefrencing {}", val),
  }

  let not_a_refrence = 3;
  let ref a_refrence = not_a_refrence;

  println!("A refrence value {}", a_refrence);

  let value = 5;
  let mut mut_value = 6;

  match value {
    ref r => println!("Got a reference to value: {}", r),
  }

  match mut_value {
    ref mut r => {
      *r += 10;
      println!("After adding 10 to mut value: {}", r);
    },
  }
}