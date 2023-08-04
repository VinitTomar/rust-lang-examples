enum WebEvents {
  PageLoad,
  PageUnload,
  KeyPress(char),
  Paste(String),
  Click { x: f32, y: f32 },
}

fn inspect(event: WebEvents) {
  match event {
    WebEvents::PageLoad => println!("page loaded"),
    WebEvents::PageUnload => println!("Page unloaded"),
    WebEvents::KeyPress(c) => println!("Key {} is pressed", c),
    WebEvents::Paste(s) => println!("You pasted: {}", s),
    WebEvents::Click {x, y} => println!("You click at x: {}, y: {}", x, y),
  }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: f32, y: f32) -> f32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
  let page_load_event = WebEvents::PageLoad;
  let page_unload_event = WebEvents::PageUnload;
  let key_press_event = WebEvents::KeyPress('k');
  let paste_event = WebEvents::Paste("Some text pasted".to_string());
  let click_event = WebEvents::Click { x: 4.3, y: 5.2 };

  inspect(page_load_event);
  inspect(page_unload_event);
  inspect(key_press_event);
  inspect(paste_event);
  inspect(click_event);

  let sum = Operations::run(&Operations::Add, 4.0, 5.6);
  println!("4.0 + 5.6 is {}", sum);

  let diff = Operations::run(&Operations::Subtract, 6.0, 2.0);
  println!("6.0 - 2.0 is {}", diff);
}