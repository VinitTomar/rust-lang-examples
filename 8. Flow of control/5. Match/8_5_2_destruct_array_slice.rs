fn main() {
  let arr = [3, 2, 4, 5, 6];

  match arr {
    [3, _, third, ..] => println!("Array => third: {}", third),
    [1, second, third, ..] => println!("Array => second: {}, third, {}", second, third),
    [first, 2, tail @ ..] => println!("Array => first: {}, tail: {:?}", first, tail),
    [first, middle @ .., last] => println!("Array => first: {}, middle: {:?}, last: {}", first, middle, last),
  }
}