use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use regex::Regex;

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d01/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}


pub fn p1() {
    let regex_matching_digits: regex::Regex = Regex::new(r"\d").unwrap();
    let puzzle_input = read_puzzle_input();
    let puzzle_input: Vec<String> = puzzle_input
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let first_and_last_number_concat: Vec<u16> = puzzle_input
        .iter()
        .map(|line| {
            let matches = regex_matching_digits.find_iter(line).collect::<Vec<_>>();
            let first = matches.get(0).unwrap().as_str();
            let last = matches.get(matches.len()-1).unwrap().as_str();
            (first, last)
        })
        .map(|(first, last)| (first.to_string()+&last).parse::<u16>().unwrap())
        .collect();
    let result: u16 = first_and_last_number_concat.iter().fold(0, |acc, x| acc + x);
    println!("sum of concatenation of first and last digit in string: {}", result);
}

fn reverse_str(inp: &str) -> String {
    inp.chars().rev().collect()
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let puzzle_input: Vec<String> = puzzle_input
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    let words = vec!["one","two","three","four","five","six","seven","eight","nine"];
    let regex_matching_digits_or_digitnames: regex::Regex = Regex::new(&format!("(\\d|{})", words.join("|"))).unwrap();
    
    let reverse_words: Vec<String> = words.iter().map(|x| reverse_str(x)).collect();
    let reverse_regex_matching_digits_or_digitnames: regex::Regex = Regex::new(&format!("(\\d|{})", reverse_words.join("|"))).unwrap();
    
    let mut numeric_value_map = HashMap::new();
    words.iter().enumerate().for_each(|(index, digit)| {
        numeric_value_map.insert(digit, index + 1);
    });
    let numeric_value_map = numeric_value_map.clone();

    let first_and_last_number_concat: Vec<u16> = puzzle_input
        .iter()
        .map(|line| {
            let first = regex_matching_digits_or_digitnames.captures(line).unwrap().get(0).unwrap().as_str().to_string();
            let reversed_line: String = line.chars().rev().collect();
            let last = reverse_regex_matching_digits_or_digitnames.captures(reversed_line.as_str()).unwrap().get(0).unwrap().as_str();
            let last: String = last.chars().rev().collect();
            (first, last)
        })
        .map(|(first, last)| {
            //println!("{first}-{last}");
            let first: String = match first.parse::<u16>() {
                Ok(val) => {val.to_string()},
                Err(_) => {numeric_value_map.get(&first.as_str()).unwrap().to_string()},
            };
            let last: String = match last.parse::<u16>() {
                Ok(val) => {val.to_string()},
                Err(_) => {numeric_value_map.get(&last.as_str()).unwrap().to_string()},
            };
            let combined_numeric = first+&last;
            //println!("{combined_numeric}");
            combined_numeric.parse::<u16>().unwrap()
        })
        .collect();
    let result: u16 = first_and_last_number_concat.iter().fold(0, |acc, x| acc + x);
    println!("sum of concatenation of first and last digit in string: {}", result);
}