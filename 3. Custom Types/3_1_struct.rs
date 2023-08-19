// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// A unit struct
struct Unit;

struct Point {
  x: f32,
  y: f32,
}

struct Pair(i32, f32);

struct Rectange {
  top_left: Point,
  botton_right: Point,
}

fn rect_area(rect: Rectange) -> f32 {
  let Rectange {
    top_left: Point {x: x1, y: y1},
    botton_right: Point {x: x2, y: y2},
  } = rect;

  let width: f32 = y2 - y1;
  let height: f32 = x1 - x2;

  width * height
}

fn main() {
  let name = String::from("Vinit");
  let age = 27;

  let person = Person { name, age };
  println!("{:?}", person);

  let point: Point = Point { x: 5.3, y: 6.2 };
  println!("coordinates = ({}, {})", point.x, point.y);

  let Point { x: left_edge, y: right_edge } = point;
  println!("left edge: {}, right edge: {}", left_edge, right_edge);

  let _unit = Unit;

  let pair = Pair(1, 0.3);
  println!("pair contains {} & {}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;
  println!("pair contains {} & {}", integer, decimal);

  let rectangle = Rectange {
    top_left: point,
    botton_right: { Point { x: 3.3, y: 10.2 } }
  };

  println!("Rectangle ares: {:.1}", rect_area(rectangle));
}