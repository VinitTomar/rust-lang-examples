use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} + {}i", self.real, self.imag)
  }
}

fn main() {
  let minmax = MinMax(2, 30);
  println!("Compare MinMax");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-100, 200);
  let small_range = MinMax(50, 60);
  println!("The big range is {big}, and small range is {small}", big=big_range, small=small_range);

  let point = Point2D {x: 3.5, y: 10.1};
  println!("Compare Point2D");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  let cplx = Complex {real: 4.2, imag: 2.1};
  println!("Compare Complex");
  println!("Display: {}", cplx);
  println!("Debug: {:?}", cplx);
}