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

mod d04 {
  pub mod solution;
}
use d04::solution::p1 as solutionD04P1;
use d04::solution::p2 as solutionD04P2;

mod d05 {
  pub mod solution;
}
use d05::solution::p1 as solutionD05P1;
use d05::solution::p2 as solutionD05P2;


mod d06 {
  pub mod solution;
}
use d06::solution::p1 as solutionD06P1;
use d06::solution::p2 as solutionD06P2;

mod d07 {
  pub mod solution;
  pub mod solution_part_2;
}
use d07::solution::p1 as solutionD07P1;
use d07::solution_part_2::p2 as solutionD07P2;

mod d08 {
  pub mod solution;
}
use d08::solution::p1 as solutionD08P1;
use d08::solution::p2 as solutionD08P2;

mod d09 {
  pub mod solution;
}
use d09::solution::p1 as solutionD09P1;
use d09::solution::p2 as solutionD09P2;

mod d10 {
  pub mod solution;
}
use d10::solution::p1 as solutionD10P1;
use d10::solution::p2 as solutionD10P2;

mod d11 {
  pub mod solution;
}
use d11::solution::p1 as solutionD11P1;
use d11::solution::p2 as solutionD11P2;

mod d12 {
  pub mod solution;
  pub mod solution_part_2;
}
use d12::solution::p1 as solutionD12P1;
use d12::solution_part_2::p2 as solutionD12P2;

mod d13 {
  pub mod solution;
}
use d13::solution::p1 as solutionD13P1;
use d13::solution::p2 as solutionD13P2;

mod d14 {
  pub mod solution;
}
use d14::solution::p1 as solutionD14P1;
use d14::solution::p2 as solutionD14P2;

mod d15 {
  pub mod solution;
}
use d15::solution::p1 as solutionD15P1;
use d15::solution::p2 as solutionD15P2;

mod d16 {
  pub mod solution;
}
use d16::solution::p1 as solutionD16P1;
use d16::solution::p2 as solutionD16P2;

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
  map.insert("d04p1".to_string(), solutionD04P1 as fn());
  map.insert("d04p2".to_string(), solutionD04P2 as fn());
  map.insert("d05p1".to_string(), solutionD05P1 as fn());
  map.insert("d05p2".to_string(), solutionD05P2 as fn());
  map.insert("d06p1".to_string(), solutionD06P1 as fn());
  map.insert("d06p2".to_string(), solutionD06P2 as fn());
  map.insert("d07p1".to_string(), solutionD07P1 as fn());
  map.insert("d07p2".to_string(), solutionD07P2 as fn());
  map.insert("d08p1".to_string(), solutionD08P1 as fn());
  map.insert("d08p2".to_string(), solutionD08P2 as fn());
  map.insert("d09p1".to_string(), solutionD09P1 as fn());
  map.insert("d09p2".to_string(), solutionD09P2 as fn());
  map.insert("d10p1".to_string(), solutionD10P1 as fn());
  map.insert("d10p2".to_string(), solutionD10P2 as fn());
  map.insert("d11p1".to_string(), solutionD11P1 as fn());
  map.insert("d11p2".to_string(), solutionD11P2 as fn());
  map.insert("d12p1".to_string(), solutionD12P1 as fn());
  map.insert("d12p2".to_string(), solutionD12P2 as fn());
  map.insert("d13p1".to_string(), solutionD13P1 as fn());
  map.insert("d13p2".to_string(), solutionD13P2 as fn());
  map.insert("d14p1".to_string(), solutionD14P1 as fn());
  map.insert("d14p2".to_string(), solutionD14P2 as fn());
  map.insert("d15p1".to_string(), solutionD15P1 as fn());
  map.insert("d15p2".to_string(), solutionD15P2 as fn());
  map.insert("d16p1".to_string(), solutionD16P1 as fn());
  map.insert("d16p2".to_string(), solutionD16P2 as fn());
  
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
