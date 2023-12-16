use std::{fs::{self, File}, io::Write, collections::HashMap};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d14/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn get_col_major_grid(str_: String) -> Vec<Vec<char>> {

    let grid = str_.split("\n").map(|line| line.to_owned()).collect::<Vec<String>>();
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut transposed_grid: Vec<Vec<char>> = vec![vec![' '; rows]; cols];
    
    for i in 0..rows {
        let chars: Vec<char> = grid[i].chars().collect();
        if !chars.is_empty() {
            for j in 0..cols {
                if j < chars.len() {
                    transposed_grid[j][i] = chars[j];
                }
            }
        }
    }

    transposed_grid
}

fn tilt_col_north(column: String) -> String {
    let column = column.split("#").map(|x| x.to_owned()).collect::<Vec<String>>();
    let tilted = column.iter().map(|chunk| {
        let mut char_vec = chunk.chars().collect::<Vec<char>>();
        char_vec.sort();
        char_vec.into_iter().rev().collect::<String>()
    }).collect::<Vec<String>>();
    tilted.join("#")
}

fn value_of_column(column: String) -> usize {
    column.chars().rev().enumerate().map(|(idx, char_)| {
        if char_ == '#' || char_ == '.' {0} else {idx + 1}
    }).fold(0, |acc, iter| acc + iter)
}

fn transpose(input: Vec<String>) -> Vec<String> {
    let max_len = input.iter().map(|x| x.len()).max().unwrap_or(0);

    let padded: Vec<Vec<char>> = input.into_iter()
        .map(|x| {
            let mut chars: Vec<char> = x.chars().collect();
            while chars.len() < max_len {
                chars.push(' ');
            }
            chars
        })
        .collect();

    (0..max_len)
        .map(|i| padded.iter().map(|x| x[i]).collect())
        .collect()
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let grid = get_col_major_grid(puzzle_input);
    let tilted_columns = grid.iter().map(|column| tilt_col_north(column.into_iter().collect::<String>()));

    let mut file = File::create("src/d14/tilted_grid.txt").expect("Could not create file");
    let transposed = transpose(tilted_columns.clone().collect::<Vec<String>>());
    for row in transposed {
        let mut line = row;
        line.push('\n');
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }
    
    let result = tilted_columns.fold(0, |acc, iter| acc + value_of_column(iter));
    println!("{:?}", result);
}

fn _rotate_right(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    if n == 0 {
        return grid;
    }
    let m = grid[0].len();
    let mut new_grid = vec![vec![' '; n]; m];
    
    for i in 0..n {
        for j in 0..m {
            new_grid[j][n - i - 1] = grid[i][j];
        }
    }
    new_grid
}

fn rotate_left(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    if n == 0 {
        return grid;
    }
    
    let m = grid[0].len();
    let mut new_grid = vec![vec![' '; m]; n];
    
    for i in 0..n {
        for j in 0..m {
            new_grid[m - j - 1][i] = grid[i][j];
        }
    }
    new_grid
}


fn transpose_grid(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let max_len = input.iter().map(|x| x.len()).max().unwrap_or(0);

    let padded: Vec<Vec<char>> = input.into_iter()
        .map(|x| {
            let mut chars = x.clone();
            while chars.len() < max_len {
                chars.push(' ');
            }
            chars
        })
        .collect::<Vec<Vec<_>>>();

    (0..max_len)
        .map(|i| padded.iter().map(|x| x[i]).collect())
        .collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("\n");
    transpose_grid(grid).iter().for_each(|first| println!("{:?}", first.into_iter().collect::<String>()));
    println!("\n");
}

fn tilt_grid_north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|column| {
        tilt_col_north(column.into_iter().collect::<String>()).chars().into_iter().collect::<Vec<char>>()
    }).collect()
}

pub fn p2() {
    let puzzle_input: String = read_puzzle_input();
    let mut grid = get_col_major_grid(puzzle_input);
    println!("original grid:");
    print_grid(&grid);
    let mut grids: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let it_in_question = 1_000_000_000;
    let mut iterations_to_go = 0;
    let mut loop_len = 0;
    grids.insert(grid.clone(), 0);
    for i in 1..it_in_question {
        for _k in 0..=3 {
            grid = tilt_grid_north(&grid);
            grid = rotate_left(grid);
        }
        match grids.insert(grid.clone(), i) {
            Some(old_idx) => {
                loop_len = i - old_idx;
                iterations_to_go = (it_in_question - i) % loop_len;
                println!("found same grid at cycle {old_idx} and {i}");
                // println!("iterations to go to equal {it_in_question}: {iterations_to_go}");
                break;
            },
            _ => {
            },
        }
    }
    println!("iterations to go: {iterations_to_go}");
    for i in 0..=loop_len {
        for _k in 0..=3 {
            grid = tilt_grid_north(&grid);
            grid = rotate_left(grid);
        }
        if i == iterations_to_go-1 {
            let result = grid.iter().fold(0, |acc, iter| acc + value_of_column(iter.into_iter().collect::<String>()));
            println!("{i}: {result}");
        }
    }
}