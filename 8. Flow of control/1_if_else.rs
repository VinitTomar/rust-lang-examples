fn main() {
  let n = 51;

  if n > 0 {
    print!("{} is positive", n);
  } else if n < 0 {
    print!("{} is negative", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = 
    if n < 10 && n > -10 {
      println!(", and is a small number, increase ten fold");
      n * 10
    } else {
      println!(", and is a big number, half of value");
      n / 2
    };
  
  println!("{} -> {}", n , big_n);
}