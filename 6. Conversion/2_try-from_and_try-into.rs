use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
  type Error = ();

  fn try_from(item: i32) -> Result<Self, Self::Error> {
    if item % 2 == 0 {
      return Ok(EvenNumber(item));
    } else {
      return Err(());
    }
  }
}

fn main() {
  // TryFrom trait
  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
  assert_eq!(EvenNumber::try_from(5), Err(()));

  // TryInto trait
  let result: Result<EvenNumber, ()> = 8_i32.try_into();
  assert_eq!(result, Ok(EvenNumber(8)));
  
  let result: Result<EvenNumber, ()> = 5_i32.try_into();
  assert_eq!(result, Err(()));
}