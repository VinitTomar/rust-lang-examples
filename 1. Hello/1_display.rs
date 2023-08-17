fn main() {
  println!("{} days", 30);

  // Positional arguments
  println!("{0}, this is {1}. {1} this is {0}", "Vin", "Tom");

  // Named arguments
  println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

  // Using format character ":"
  println!("Base 10:               {}",   69420); // 69420
  println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420); // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
  println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

  // You can right-justify text with a specified width. This will
  // output "    1". (Four white spaces and a "1", for a total width of 5.)
  println!("{number:>5}", number=1);

  // You can pad numbers with extra zeroes,
  println!("{number:0>5}", number=1); // 00001
  // and left-adjust by flipping the sign. This will output "10000".
  println!("{number:0<5}", number=1); // 10000

  // You can use named arguments in the format specifier by appending a `$`.
  println!("{number:0>width$}", number=6, width=3);

  let number: f64 = 1.0;
  let width: usize = 5;
  println!("{number:>width$}");

  // #[allow(dead_code)] // disable `dead_code` which warn against unused module
  // struct Structure(i32);

  // This will not compile because `Structure` does not implement
  // fmt::Display.
  // println!("This struct `{}` won't print...", Structure(3));
  // ^ Try uncommenting this line

  let pi = 3.141592;
  println!("{pi:.3}")
}