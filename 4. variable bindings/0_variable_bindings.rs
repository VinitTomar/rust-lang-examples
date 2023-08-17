fn main() {
  let an_integer = 1u32;
  let a_boolean = false;
  let a_unit = ();

  let copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit value: {:?}", a_unit);

  let _unused_variable = 3u32;
  // Prefix with an underscore to suppress the warning
}