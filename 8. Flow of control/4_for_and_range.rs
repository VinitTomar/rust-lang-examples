
fn example_iter() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter() {
    match name {
      &"Frank" => println!("There is a rustacean among us"),
      _ => println!("Hello {}", name),
    }
  }

  println!("names: {:?}", names);
}

fn example_into_iter() {
  let names = vec!["Bob", "Frank", "Ferris"];

  for name in names.into_iter() {
    match name {
      "Frank" => println!("There is a rustacean among us"),
      _ => println!("Hello {}", name),
    }
  }

  // using names will throw error
  // println!("names: {:?}", names);
}

fn example_iter_mut() {
  let mut names = vec!["Bob", "Frank", "Ferris"];

  for name in names.iter_mut() {
    *name = match name {
      &mut "Ferris" => "There is a rustacean among us",
      _ => "Hello",
    }
  }

  println!("names: {:?}", names);
}

fn main() {
  for n in 1..=10 {
     if n % 15 == 0 {
      println!("Fizzbuzz");
    } else if n % 5 == 0 {
      println!("Fizz");
    } else if n % 3 == 0 {
      println!("Buzz");
    } else {
      println!("{}", n);
    }
  }

  example_iter();
  example_into_iter();
  example_iter_mut();
}