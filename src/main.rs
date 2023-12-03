use std::process::exit;
use std::env::args;
use std::collections::HashMap;

fn main() {
  let args: Vec<String> = args().collect();
  let mut day = None;
  let mut part = None;

  let mut i = 1;
  while i < args.len() {
      match args[i].as_str() {
          "--day" => {
              i += 1;
              if i < args.len() {
                  day = Some(args[i].clone());
              }
          }
          "--part" => {
              i += 1;
              if i < args.len() {
                  part = Some(args[i].clone());
              }
          }
          _ => {}
      }
      i += 1;
  }

  let (day, part) = match (day, part) {
    (Some(day), Some(part)) => (if day.len() == 1 { format!("0{}",day) } else { day }, part),
    _ => {
        println!("Usage: cargo run -- --day <> --part <>");
        std::process::exit(1);
    }
  };

  let mut map: HashMap<String, fn()> = HashMap::new();
  
  let func_name_to_query = format!("d{day}p{part}");
  println!("func name to query map with: {}",func_name_to_query);
  let func = match map.get(func_name_to_query.as_str()) {
      Some(func) => {func},
      None => {
        println!("did not find a function for this parameters");
        exit(0)
      },
  };
  
  println!("-----Begin of function-----");
  func();
  println!("-----End of function-----");
}
