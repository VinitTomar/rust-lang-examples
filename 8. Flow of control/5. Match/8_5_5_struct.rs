#[allow(unreachable_patterns)]

fn main() {
  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  let foo = Foo{x: (5, 2), y: 3};

  match foo {
    Foo {x: (1, b), y} => println!("X > (1, {b}), Y > {y}"),
    Foo {y, x: i } => println!("X > {:?}, Y > {y}", i),
    Foo {y, ..} => println!("Only y > {y}"),
  }

}