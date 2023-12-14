use std::fs::{self};
extern crate indicatif;
use indicatif::{ProgressBar, ProgressStyle};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d12/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn rle(input: &str) -> Vec<usize> {
    let mut count = 0;
    let mut current = '\0';
    let mut output: Vec<usize> = Vec::new();

    for ch in input.chars() {
        if ch != current {
            if current != '\0' {
                output.push(count);
            }
            count = 1;
            current = ch;
        } else {
            count += 1;
        }
    }
    output.push(count);
    
    if input.chars().next().unwrap() == '#' {
        output.insert(0, 0);
    }

    output
}

#[cfg(test)]
mod rle_tests {
    use super::*;

    #[test]
    fn test_rle() {
        assert_eq!(rle(".###.##.#..."), vec![1, 3, 1, 2, 1, 1, 3]);
        assert_eq!(rle("#.#.###"), vec![0, 1, 1, 1, 1, 3]);
        assert_eq!(rle(".#..##..."), vec![1, 1, 2, 2, 3]);
        assert_eq!(rle("#"), vec![0, 1]);
    }
}

fn get_damaged_groups<T>(vec: &Vec<T>) -> Vec<&T> {
    vec.into_iter().enumerate().filter(|&(i, _)| i % 2 != 0).map(|(_, item)| item).collect::<Vec<_>>()
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let lines = puzzle_input.split("\n").map(|line| {
        let mut split = line.split_whitespace();
        let first = split.next().unwrap();
        let second = split.next().unwrap().split(',').into_iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        (first, second)
    }).collect::<Vec<(&str, Vec<usize>)>>();
    println!("lines: {}", lines.len());
    
    let total_lines = lines.len();
    let bar = ProgressBar::new(total_lines as u64);
    bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {pos}/{len}"));
    
    let mut result = 0;
    for (i, (string, expected_value)) in lines.iter().enumerate() {
        //println!("\nline: {i}");
        let permutations = generate_permutations(string);
        //permutations.iter().for_each(|permutation| println!("{:?}", permutation));
        let mut matching_permutations_count = 0;
        
        for permutation in &permutations {
            if keep_permutations_matching_expected_value(permutation, expected_value) {
                matching_permutations_count += 1;
            }
        }
        result += matching_permutations_count;
    
        // increment the progress bar
        bar.inc(1);
    }
    
    // finish the progress bar
    bar.finish();
    println!("result: {:?}{}", result, if result >= 65874 {" which is too high" } else {""});
}

fn keep_permutations_matching_expected_value(permutation: &String, expected_value: &[usize]) -> bool {
    let rle = rle(permutation);
    let damaged_groups = get_damaged_groups(&rle);
    if damaged_groups.len() != expected_value.len() { return false; }
    let result = expected_value.iter().zip(damaged_groups).all(|(&x, &y)| x == y);
    result
}

pub fn generate_permutations(string: &str) -> Vec<String> {
    let bits = string.matches('?').count();
    let iterations = 2usize.pow(bits as u32);
    let mut permutations = Vec::with_capacity(iterations);
    for n in 0..iterations {
        let binary = format!("{:0width$b}", n, width = bits);
        let mut result = String::from(string);
        binary.chars().rev().collect::<Vec<char>>().iter().enumerate().for_each(|(i,b)| {
            if *b == '0' {
                result = result.replacen("?", ".", 1);
            } else {
                result = result.replacen("?", "#", 1);
            }
        });
        permutations.push(result);
    }
    permutations
}
