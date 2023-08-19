use std::mem;

fn analyze_slice(slice: &[i32]) {
  println!("Slice first elm: {}", slice[0]);
  println!("Slice length: {}", slice.len());
}

fn main() {
  let xs: [i32; 5] = [1, 2, 3, 4, 5];
  // To fill array with zeros;
  let ys: [i32; 500] = [0; 500];

  println!("First elm of xs: {}", xs[0]);
  println!("Second elm of xs: {}", xs[1]);
  println!("Total elements in xs: {}", xs.len());
  println!("Memory occupied (in bytes) by xs: {}", mem::size_of_val(&xs));

  println!("Borrow the whole array as a slice.");
  analyze_slice(&xs);

  println!("Borrow a section of array");
  analyze_slice(&ys[4..9]);

  let empty_array: [u32; 0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]);

  for i in 0..xs.len() + 1 {
    match xs.get(i) {
      Some(xval) => println!("{}: {}", i, xval),
      None => println!("Hold on, {} is too far away", i),
    }
  }

  // Out of bound indexing on array causes compile time error.
  // println!("{}", xs[5]);
  // Out of bound indexing on slice causes runtime error.
  // println!("{}", xs[..][5]);
}