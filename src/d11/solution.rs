use std::{fs::{self}};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d11/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

type Grid<'a> = Vec<Vec<char>>;

fn find_empty_rows(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut row_indices = vec![];
    
    for (i, row) in matrix.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            row_indices.push(i);
        }
    }
    
    row_indices
}

fn find_empty_columns(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let mut col_indices = vec![];
    
    if !matrix.is_empty() {
        for j in 0..matrix[0].len() {
            if matrix.iter().all(|row| row[j] == '.') {
                col_indices.push(j);
            }
        }
    }
    
    col_indices
}

fn pad_matrix(matrix: &Vec<Vec<char>>, row_indices: &Vec<usize>, col_indices: &Vec<usize>) -> Vec<Vec<char>> {
    let mut new_matrix = matrix.clone();
    
    for &i in row_indices.iter().rev() {
        let row = matrix[i].clone();
        new_matrix.insert(i+1, row);
    }
    
    for &j in col_indices.iter().rev() {
        for row in new_matrix.iter_mut() {
            let elem = row[j];
            row.insert(j+1, elem);
        }
    }

    new_matrix
}

fn find_galaxies(rows: &Grid) -> Vec<(usize, usize)>{ 
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, char_) in row.into_iter().enumerate() {
            if *char_ == '#' { galaxies.push((row_idx, col_idx)) }
        }
    }
    galaxies
}

fn pair_galaxies<T>(galaxies: &Vec<(T, T)>) -> Vec<((T, T), (T, T))> 
where T: Copy
{
    let mut result = Vec::new();
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            result.push((galaxies[i], galaxies[j]));
        }
    }
    result
}

fn abs_diff<T>(input: &Vec<((T, T), (T, T))>) -> Vec<(T, T)> where T: std::cmp::Ord + std::ops::Sub<Output=T> + Copy {
    let mut result = Vec::new();
    for item in input {
        let x_diff = if item.0 .0 > item.1 .0 { item.0 .0 - item.1 .0 } else { item.1 .0 - item.0 .0 };
        let y_diff = if item.0 .1 > item.1 .1 { item.0 .1 - item.1 .1 } else { item.1 .1 - item.0 .1 };
        result.push((x_diff, y_diff));
    }
    result
}

fn sum_abs_diff<T>(input: &Vec<((T, T), (T, T))>) -> Vec<T> 
where T: std::cmp::Ord + std::ops::Sub<Output = T> + std::ops::Add<Output = T> + Copy {
    let mut result = Vec::new();
    for item in input {
        let x_diff = if item.0 .0 > item.1 .0 { item.0 .0 - item.1 .0 } else { item.1 .0 - item.0 .0 };
        let y_diff = if item.0 .1 > item.1 .1 { item.0 .1 - item.1 .1 } else { item.1 .1 - item.0 .1 };
        result.push(x_diff + y_diff);
    }
    result
}

pub fn p1() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    //println!("grid: {:?}", grid);
    //grid.iter().for_each(|row| println!("{:?}", row));
    let columns_to_expand = find_empty_columns(&grid);
    let rows_to_expand = find_empty_rows(&grid);
    let grid = pad_matrix(&grid, &rows_to_expand, &columns_to_expand);
    //println!("grid: {:?}", grid);
    grid.iter().for_each(|row| println!("{:?}", row));
    let galaxies = find_galaxies(&grid);
    println!("{galaxies:?}");
    let pairs = pair_galaxies(&galaxies);
    println!("{pairs:?}");
    let diff = sum_abs_diff(&pairs);
    println!("{diff:?}");
    println!("result: {:?}", diff.iter().fold(0, |acc, x| acc + x));
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    //println!("grid: {:?}", grid);
    //grid.iter().for_each(|row| println!("{:?}", row));
    let columns_to_expand = find_empty_columns(&grid);
    let rows_to_expand = find_empty_rows(&grid);
    let galaxies = find_galaxies(&grid);
    println!("{galaxies:?}");
    //let expansion_coefficient: u128 = 1_000_000;
    let expansion_coefficient: u128 = 1_000_000 - 1;
    let galaxies = galaxies.iter()
        .map(|(row, col)| (*row as u128, *col as u128))
        .map(|(row, col)| (
            row + rows_to_expand.iter().filter(|row_| row > (**row_ as u128)).collect::<Vec<_>>().len() as u128 * expansion_coefficient,
            col + columns_to_expand.iter().filter(|col_| col > (**col_ as u128)).collect::<Vec<_>>().len() as u128 * expansion_coefficient
        ))
        .collect::<Vec<(u128, u128)>>();
    let pairs = pair_galaxies(&galaxies);
    println!("\n{pairs:?}");
    println!("\n{:?} pairs", pairs.len());
    let diff = sum_abs_diff(&pairs);
    println!("\n{diff:?}\n");
    println!("\nresult: {:?}", diff.iter().fold(0, |acc, x| acc + x));
}