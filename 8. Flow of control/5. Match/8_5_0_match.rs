fn main() {
  let number = 13;

  println!("Something about number: {}", 13);

  match number {
    1 => println!("This is one"),
    2 => println!("This is two"),
    3 | 5 | 7 | 11 | 13 => println!("This is prime number"),
    13..=19 => println!("Team A number"),
    _ => println!("Not a special number"),
  }

  let boolean = true;

  let binary = match boolean {
    true => 1,
    false => 0,
  };

  println!("{} => {}", boolean, binary);
}