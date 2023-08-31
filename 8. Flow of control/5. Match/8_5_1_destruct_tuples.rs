fn main() {
  let tuple = (1, -1, 4, 3);

  match tuple {
    (0, .., y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
    (4, ..) => println!("First is 4 and the rest dosen't matter"),
    (.., 3) => println!("Last is 3 and the rest dosen't matter"),
    (1, .., 5) => println!("Fist is 1, Last is 5 and the rest dosen't matter"),
    _ => println!("It dosen't matter what they are"),
  }
}