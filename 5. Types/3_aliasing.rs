type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
  let nanosec: NanoSecond = 5 as U64;
  let inch: Inch = 10 as U64;

  println!("{} nanoseconds + {} inches = {} unit?", nanosec, inch, nanosec + inch);

}