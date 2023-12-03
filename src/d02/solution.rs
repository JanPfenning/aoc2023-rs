use std::fs;
use std::path::PathBuf;

use regex::Regex;

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d02/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

#[derive(Debug)]
pub struct Draw {
    red: u8, green: u8, blue: u8
}

#[derive(Debug)]
pub struct Game {
    id: u8,
    draws: Vec<Draw>
}

fn get_numeric_id_from_game_meta_string_part(meta_string: &str) -> u8 {
    let chars = meta_string.chars().collect::<Vec<char>>();
    chars[5..].iter().collect::<String>().parse::<u8>().unwrap()
}

fn get_draw_from_string(single_draw_string: &str) -> Draw {
    let regex_matching_number = Regex::new(r"^\d+").unwrap();
    let regex_matching_red_segment = Regex::new(r"\d+ red").unwrap();
    let regex_matching_blue_segment = Regex::new(r"\d+ blue").unwrap();
    let regex_matching_green_segment = Regex::new(r"\d+ green").unwrap();
    let red = regex_matching_red_segment.captures(single_draw_string).and_then(|value| regex_matching_number.captures(Some(value.get(0).unwrap()).unwrap().as_str()).unwrap().get(0));
    let blue = regex_matching_blue_segment.captures(single_draw_string).and_then(|value| regex_matching_number.captures(Some(value.get(0).unwrap()).unwrap().as_str()).unwrap().get(0));
    let green = regex_matching_green_segment.captures(single_draw_string).and_then(|value| regex_matching_number.captures(Some(value.get(0).unwrap()).unwrap().as_str()).unwrap().get(0));
    Draw {
        red: match red {
            Some(e) => e.as_str().parse::<u8>().unwrap(),
            None => 0,
        },
        blue: match blue {
            Some(e) => e.as_str().parse::<u8>().unwrap(),
            None => 0,
        },
        green: match green {
            Some(e) => e.as_str().parse::<u8>().unwrap(),
            None => 0,
        }
    }
}

fn parse_draws_string_to_draw_struct(draw_string: &str) -> Vec<Draw> {
    let draws: Vec<Draw> = draw_string
        .split(";")
        .collect::<Vec<&str>>()
        .iter()
        .map(|single_draw_string| get_draw_from_string(&single_draw_string))
        .collect();
    draws
}

fn parse_games() -> Vec<Game>{
    let puzzle_input = read_puzzle_input();

    let games = puzzle_input
        .split("\n")
        .map(|s| s
            .split(":")
            .collect::<Vec<&str>>()
        )
        .collect::<Vec<Vec<&str>>>();
    let games = games.iter().map(|game| {
        Game {
            id: get_numeric_id_from_game_meta_string_part(&game[0]),
            draws: parse_draws_string_to_draw_struct(&game[1]) 
        }
    }).collect::<Vec<Game>>();
    games
}

pub fn p1() {
    let max_draw = Draw {
        red: 12, green: 13, blue: 14
    };
    let games = parse_games();
    println!("{:?}", games.get(0).unwrap());
    let possible_games = games
        .iter()
        .filter(|game| game
            .draws
            .iter()
            .all(|draw| 
                    draw.blue <= max_draw.blue
                 && draw.green <= max_draw.green 
                 && draw.red <= max_draw.red
            ))
        .collect::<Vec<&Game>>();
    println!("{} possible games found", possible_games.len());
    let sum_of_possible_game_ids: u16 = possible_games.iter().map(|game| game.id as u16).fold(0, |a,b| a + b);
    println!("sum of possible game ids: {sum_of_possible_game_ids}")
}

pub fn p2() {
    let games = parse_games();
    let max_overall_draw = games
        .iter()
        .map(
            |game| game.draws
                .iter()
                .fold(Draw {red: 0, green: 0, blue: 0}, |acc, draw| 
                    Draw {
                        red: acc.red.max(draw.red),
                        blue: acc.blue.max(draw.blue),
                        green: acc.green.max(draw.green)
                    }
                )
        ).collect::<Vec<Draw>>();
    let power_of_draw = max_overall_draw.iter().map(|draw| (draw.red as usize) * (draw.green as usize) * (draw.blue as usize)).collect::<Vec<usize>>();
    let sum_of_powers_of_draws = power_of_draw.iter().fold(0, |acc,iter| acc+iter);
    println!("sum_of_powers_of_draws: {:?}",sum_of_powers_of_draws)
}