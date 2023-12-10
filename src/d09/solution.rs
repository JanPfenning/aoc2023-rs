use std::fs;
use regex::Regex;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d09/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").filter_map(|val| if val != "" {Some(val.to_owned())} else {None}).collect::<Vec<String>>();
    let whitespaces = Regex::new(r"\s+").unwrap();
    let data_points_of_functions = input_lines.iter()
        .map(
            |line| {
                let line = whitespaces.replace_all(line, ",");
                line.split(",").map(|x| {
                    x.parse::<isize>().unwrap()
                }
                ).collect::<Vec<_>>()   
            }
        )
        .collect::<Vec<Vec<_>>>();
    //println!("points: {:#?}", data_points_of_functions);
    let added_values = data_points_of_functions.iter().map(|list| {
        let extended_list = extend_list_via_derivative(list);
        let new_element = extended_list.last().unwrap();
        println!("{} as new element in {:?}", new_element, extended_list);
        *new_element
    }).collect::<Vec<isize>>();
    println!("result: {:?}", added_values.iter().fold(0, |acc, new_element| acc + new_element));
}

fn extend_list_via_derivative(list: &Vec<isize>) -> Vec<isize> {
    let mut extendable_list = list.clone();
    if list.iter().all(|x| *x == 0) {
        extendable_list.push(0);
        return extendable_list;
    }
    let derivative: Vec<isize> = list.windows(2)
        .map(|window| (window[1] - window[0]))
        .collect();
    println!("{:?}", derivative);
    let extended_derivative = extend_list_via_derivative(&derivative);
    let next_value_based_on_derivative = list.last().unwrap() + extended_derivative.last().unwrap();
    extendable_list.push(next_value_based_on_derivative);
    return extendable_list;
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").filter_map(|val| if val != "" {Some(val.to_owned())} else {None}).collect::<Vec<String>>();
    let whitespaces = Regex::new(r"\s+").unwrap();
    let data_points_of_functions = input_lines.iter()
        .map(
            |line| {
                let line = whitespaces.replace_all(line, ",");
                let line = line.split(",").map(|x| {
                    x.parse::<isize>().unwrap()
                }    
                ).collect::<Vec<_>>()  ;
                let mut reversed = line.clone();
                reversed.reverse();
                reversed
            }
        )
        .collect::<Vec<Vec<_>>>();
    //println!("points: {:#?}", data_points_of_functions);
    let added_values = data_points_of_functions.iter().map(|list| {
        let extended_list = extend_list_via_derivative(list);
        let new_element = extended_list.last().unwrap();
        println!("{} as new element in {:?}", new_element, extended_list);
        *new_element
    }).collect::<Vec<isize>>();
    println!("result: {:?}", added_values.iter().fold(0, |acc, new_element| acc + new_element));
}
