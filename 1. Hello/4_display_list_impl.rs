use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let vec = &self.0;
    write!(f, "[\n")?;

    for (i, v) in vec.iter().enumerate() {
      if i != 0 { write!(f, ",\n")?; }
      write!(f, " {}: {}",i, v)?;
    }

    write!(f, ",\n]")

  }
}

fn main() {
  let v = List(vec![1, 2, 3]);
  println!("{}", v);
}