use std::process::exit;
use std::env::args;
use std::collections::HashMap;

mod d01 {
  pub mod solution;
}
use d01::solution::p1 as solutionD01P1;
use d01::solution::p2 as solutionD01P2;

mod d02 {
  pub mod solution;
}
use d02::solution::p1 as solutionD02P1;
use d02::solution::p2 as solutionD02P2;

mod d03 {
  pub mod solution;
}
use d03::solution::p1 as solutionD03P1;
use d03::solution::p2 as solutionD03P2;

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
  map.insert("d01p1".to_string(), solutionD01P1 as fn());
  map.insert("d01p2".to_string(), solutionD01P2 as fn());
  map.insert("d02p1".to_string(), solutionD02P1 as fn());
  map.insert("d02p2".to_string(), solutionD02P2 as fn());
  map.insert("d03p1".to_string(), solutionD03P1 as fn());
  map.insert("d03p2".to_string(), solutionD03P2 as fn());
  
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
