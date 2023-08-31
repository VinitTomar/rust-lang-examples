fn main() {
  let mut n = 0;
  while n < 101 {
    if n % 15 == 0 {
      println!("Fizzbuzz");
    } else if n % 5 == 0 {
      println!("Fizz");
    } else if n % 3 == 0 {
      println!("Buzz");
    } else {
      println!("{}", n);
    }

    n += 1;
  }
}