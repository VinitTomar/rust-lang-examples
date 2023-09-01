#[allow(dead_code)]

enum Temperature {
  Celsius(i32),
  Fahrenheit(i32),
}

fn main() {
  let temperature = Temperature::Celsius(33);

  match temperature {
    Temperature::Celsius(t) if t > 32 => println!("Temperature {t}C is greater than 32"),
    Temperature::Celsius(t) => println!("Temperature {t} is less than or equal to 32"),
    Temperature::Fahrenheit(t) if t > 55 => println!("Temperature {t}F is greater than 55"),
    Temperature::Fahrenheit(t) => println!("Tempature {t}F is less than or equal to 55"),
  }
}