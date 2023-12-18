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

pub fn p2() {
    let puzzle_input = read_puzzle_input();
}