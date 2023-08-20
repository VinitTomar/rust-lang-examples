#[allow(unreachable_code)]
#[allow(unused_labels)]

fn main() {
  'outer: loop {
    println!("this is outer loop");

    'inner: loop {
      println!("this is inner loop");

      // this will break inner loop
      // break;

      break 'outer;
    }

    println!("This is unreachable code");
  }

  println!("outside of outer loop");
}