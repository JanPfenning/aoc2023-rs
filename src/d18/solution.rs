use core::panic;
use std::{fs::{self, File}, collections::VecDeque};
use std::io::prelude::*;
use regex::Regex;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d18/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn first_number_in_string(s: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    re.find(s)
        .and_then(|mat| mat.as_str().parse::<u32>().ok())
}

fn write_grid_to_file(file_path: &str, grid: &Vec<Vec<char>>) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    
    for row in grid {
        let line: String = row.into_iter().collect();
        writeln!(file, "{}", line)?;
    }
    
    Ok(())
}

pub fn p1() {
    let puzzle_input = read_puzzle_input();
    let lines = puzzle_input.split('\n').map(|line| line.to_string()).collect::<Vec<String>>();
    let (height, width) = lines.iter().fold((1,1), |(height, width), iter| {
        match iter.chars().next().unwrap() {
            'D' => {(height + first_number_in_string(iter).unwrap() * 2, width)},
            //'U' => {(height - first_number_in_string(iter).unwrap(), width)},
            'R' => {(height, width + first_number_in_string(iter).unwrap() * 2)},
            //'L' => {(height, width - first_number_in_string(iter).unwrap())},
            _ => {(height, width)},
        }
    });
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    let _write_result = write_grid_to_file("src/d18/resulting_grid.txt", &grid);
    let mut cur_coordinate: (u128, u128) = ((height/2).into(),(width/2).into());
    for (_idx, line) in lines.iter().enumerate() {
        let direction = line.chars().next();
        let amount = first_number_in_string(line).unwrap();
        
        // iterate for the amount of steps
        for _ in 0..amount {
            match direction {
                // move cur_coordinate down
                Some('D') => {
                    cur_coordinate.1 += 1;
                },
                // move cur_coordinate up
                Some('U') => {
                    cur_coordinate.1 -= 1;
                },
                // move cur_coordinate to the right
                Some('R') => {
                    cur_coordinate.0 += 1;
                },
                // move cur_coordinate to the left
                Some('L') => {
                    cur_coordinate.0 -= 1;
                },
                _ => {},
            }
            // set the value at the current coordinate to '#'
            grid[cur_coordinate.1 as usize][cur_coordinate.0 as usize] = '#';
        }
    }
    let _write_result = write_grid_to_file("src/d18/resulting_grid.txt", &grid);
    flood_fill(&mut grid, (614,1010), '#');
    let _write_result = write_grid_to_file("src/d18/resulting_grid.txt", &grid);
    println!("result: {}", grid.iter().flatten().fold(0, |acc, iter| acc + if *iter == '#' {1} else {0}))
}

fn flood_fill(grid: &mut Vec<Vec<char>>, (x, y): (usize, usize), new_char: char) {
    let old_char = '.';  // Assuming we're filling space.
    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if grid[x][y] != old_char {
            continue;
        }

        grid[x][y] = new_char;

        if x >= 1 && grid[x - 1][y] == old_char { queue.push_back((x - 1, y)); }
        if y >= 1 && grid[x][y - 1] == old_char { queue.push_back((x, y - 1)); }
        if x < grid.len() - 1 && grid[x + 1][y] == old_char { queue.push_back((x + 1, y)); }
        if y < grid[0].len() - 1 && grid[x][y + 1] == old_char { queue.push_back((x, y + 1)); }
    }
}

enum Direction { Up, Down, Right, Left }

struct Instruction {
    direction: Direction,
    value: isize,
}

fn decode(instructions: Vec<Instruction>) -> Vec<[isize; 2]> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut corners = vec![[0, 0]];

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => y += instruction.value,
            Direction::Down => y -= instruction.value,
            Direction::Right => x += instruction.value,
            Direction::Left => x -= instruction.value,
        }

        corners.push([x as isize, y as isize]);
    }

    corners.push(corners[0]);  // close the polygon
    corners
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let lines = puzzle_input.split('\n').map(|line| line.to_string()).collect::<Vec<String>>();
    let instructions = lines.iter().map(|line| {
        let re = regex::Regex::new(r"#\b[0-9A-Fa-f]{6}\b").unwrap();
        let cap = re.captures(line).unwrap();
        let hex = &cap[0][1..];    // Remove the # at the start
        let value_hex = &hex[..5]; // First 5 characters for the value
        let direction_hex = &hex[5..6]; // Last character for the direction
        let decimal = isize::from_str_radix(value_hex, 16).unwrap();
        Instruction {
            direction: match direction_hex {
                "0" => {Direction::Right},
                "1" => {Direction::Down},
                "2" => {Direction::Left},
                "3" => {Direction::Up},
                _ => {panic!("invalid direction")},
            },
            value: decimal,
        }
    }).collect::<Vec<_>>();

    let corners: Vec<[isize; 2]> = decode(instructions);
    println!("{:?}", corners);

    // Gauss's area formula https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area: isize = 0;
    for i in 0..corners.len() - 1 {
        let current = corners[i];
        let next = corners[i + 1];
        area += current[0] * next[1] - current[1] * next[0];
    }

    let mut perimeter: isize = 0;
    for i in 0..corners.len() {
        let p1 = corners[i];
        let p2 = corners[(i+1) % corners.len()];
        perimeter += (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
    }
    println!("result {}", ((area as f64).abs() / 2.0) + (perimeter as f64 / 2.0) + 1.0);
}