use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;
  (boolean, integer)
}

#[derive(Debug)]
struct Matrix2X2(f32, f32, f32, f32);

impl fmt::Display for Matrix2X2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
  }
}

fn transpose(mtx: Matrix2X2) -> Matrix2X2 {
  Matrix2X2(mtx.0, mtx.2, mtx.1, mtx.3)
}

fn main() {
  let pair = (44, true);

  // tuples can not be printed with Display.
  // they implemented Debug trait by default.
  // Therefore we do not need to do #[derive(Debug)]

  println!("Original pair is: {:?}", pair);
  println!("Reverse pair is: {:?}", reverse(pair));

  // But long Tuples (more than 12 elements) cannot be printed.
  let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  println!("Too long tubple, first element: {} & second element: {}", too_long_tuple.0, too_long_tuple.1);
  // println!("Too long tuple: {:?}", too_long_tuple);
  // ^ Uncomment the above lines to see the compiler error

  let mtx = Matrix2X2(1.1, 1.2, 2.1, 2.2);
  println!("Matrix is =>  {:?}", mtx);
  println!("Matrix:\n{}", mtx);
  println!("Transpse:\n{}", transpose(mtx));

}