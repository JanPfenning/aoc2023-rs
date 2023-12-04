use std::{fs, panic};
use std::path::PathBuf;

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d03/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

type Grid = Vec<Vec<char>>;
fn create_grid(schematic: String) -> Grid {
    schematic.split("\n").map(|line| line.chars().collect::<Vec<char>>()).collect::<Grid>()
}

#[derive(Debug)]
#[derive(Clone)]
struct PartNumber {
    row: usize,
    col: usize,
    number: usize
}

#[derive(Debug)]
#[derive(Clone)]
struct GridNumber {
    row: usize,
    col: usize,
    number: usize
}

impl From<PartNumber> for GridNumber {
    fn from(part: PartNumber) -> Self {
       GridNumber { row: part.row, col: part.col, number: part.number }
    }
}

impl From<GridNumber> for PartNumber {
    fn from(grid: GridNumber) -> Self {
        PartNumber { row: grid.row, col: grid.col, number: grid.number }
    }
}

fn parse_number(char_list: Vec<char>) -> usize {
    let string = char_list.iter().collect::<String>(); 
    string.parse::<usize>().expect(format!("could not parse the string \"{}\" to a number", string).as_str())
}

fn find_numbers(grid: &Grid) -> Vec<GridNumber> {
    let mut grid_numbers: Vec<GridNumber> = Vec::new();
    grid.iter().enumerate().for_each(|(row_index, row)| {
        let mut col_start: isize = -1;
        let mut len: usize = 0;
        row.iter().enumerate().for_each(|(col_index, current_char)| {
            let new_grid_number_found = current_char.is_numeric() && col_start == -1;
            let grid_number_continues = current_char.is_numeric() && col_start != -1;
            let grid_number_ended = !current_char.is_numeric() && col_start != -1;
            let searching_number = !current_char.is_numeric() && col_start == -1;
            if new_grid_number_found {
                col_start = col_index as isize;
                len += 1;
            } else if grid_number_continues {
                len += 1;
            } else if grid_number_ended {
                let chars_of_cur_grid_number = &row[(col_start as usize)..((col_start as usize)+len)];
                let col_start_clone = col_start;
                let mut push_grid_number = |grid_number: GridNumber| {
                    grid_numbers.push(grid_number);
                    col_start = -1;
                    len = 0;
                };
                push_grid_number(GridNumber { row: (row_index), col: (col_start_clone as usize), number: parse_number(chars_of_cur_grid_number.to_vec()) })
            } else if searching_number {
                //
            } else {
                panic!("impossible state")
            }
            // At line end => if there is a digit check if it spart of a greater one or single digit and add resulting number too
            if col_index == row.len()-1  && current_char.is_numeric() {
                if col_start == -1 {
                    let mut push_grid_number = |grid_number: GridNumber| {
                        grid_numbers.push(grid_number);
                        col_start = -1;
                        len = 0;
                    };
                    push_grid_number(GridNumber { row: (row_index), col: (col_index), number: parse_number(vec![current_char.clone()]) })
                } else {
                    let chars_of_cur_grid_number = &row[(col_start as usize)..((col_start as usize)+len)];
                let col_start_clone = col_start;
                let mut push_grid_number = |grid_number: GridNumber| {
                    grid_numbers.push(grid_number);
                    col_start = -1;
                    len = 0;
                };
                push_grid_number(GridNumber { row: (row_index), col: (col_start_clone as usize), number: parse_number(chars_of_cur_grid_number.to_vec()) })
                }
            }
        })
    });
    grid_numbers
}

fn has_symbol_around(grid: &Grid, grid_number: &GridNumber) -> bool {
    println!("cur processed grid_number: {:?}", grid_number);
    let rows = match panic::catch_unwind(|| grid_number.row-1) {
        Ok(val) => {val},
        Err(_) => {0},
    }..=grid_number.row+1;
    let cols = match panic::catch_unwind(|| grid_number.col-1) {
        Ok(val) => {val},
        Err(_) => {0},
    }..=grid_number.col+grid_number.number.to_string().chars().into_iter().collect::<Vec<_>>().len();
    //println!("rows: {:?}, cols: {:?}", rows, cols);
    let chars: Vec<&char> = rows.filter_map(|row_index| {
        grid.get(row_index).map(|row| {
            //println!("row {:?}", row);
            cols.clone().filter_map(|col_index| row.get(col_index)).collect::<Vec<_>>()
        })
    })
    .flatten()
    .filter(|char_val| !char_val.is_numeric() && !char_val.eq(&&'.'))
    .collect();
    //println!("{:?}", chars);
    chars.len() > 0
}

pub fn p1() {
    let grid = create_grid(read_puzzle_input());
    let grid_numbers = find_numbers(&grid);
    let part_numbers = grid_numbers.iter().filter(|grid_number| has_symbol_around(&grid, &grid_number)).map(|grid_number| (*grid_number).clone().into()).collect::<Vec<PartNumber>>();
    println!("part_numbers.len() {}", part_numbers.len());
    let sum_of_part_numbers_values = part_numbers.iter().fold(0, |acc, iter| acc + iter.number);
    println!("sum_of_part_numbers_values {sum_of_part_numbers_values}");
}

pub fn p2() {
    println!("{}", read_puzzle_input())
}
