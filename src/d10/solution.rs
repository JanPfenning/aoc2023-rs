use std::{fs::{self, File}, collections::HashMap, io::Write};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d10/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

type Grid<'a> = Vec<Vec<char>>;

fn find_s(rows: &Grid) -> (usize, usize){ 
    for (row_idx, row) in rows.iter().enumerate() {
        for (col_idx, char_) in row.into_iter().enumerate() {
            if *char_ == 'S' { return (row_idx, col_idx) }
        }
    }
    panic!("did not find 'S'");
}

type Triple<'a> = Option<(&'a char, usize, usize)>;

fn get_surroundings<'a>(grid: &'a Grid<'a>, cur_pos: (usize, usize)) -> (Triple<'a>, Triple<'a>, Triple<'a>, Triple<'a>){
    // Up
    let up: Triple = if cur_pos.0 > 0 { 
        grid.get(cur_pos.0-1)
            .and_then(|row| row.get(cur_pos.1)
            .map(|char_| (char_, cur_pos.0-1, cur_pos.1)))
    } else { None };

    // Right
    let right: Triple = grid.get(cur_pos.0)
        .and_then(|row| if cur_pos.1 < row.len() - 1 { row.get(cur_pos.1+1) } else { None }
        .map(|char_| (char_, cur_pos.0, cur_pos.1+1)));

    // Down
    // Down
    let down: Triple = if cur_pos.0 < grid.len() - 1 { 
        grid.get(cur_pos.0+1)
            .and_then(|row| row.get(cur_pos.1)
            .map(|char_| (char_, cur_pos.0+1, cur_pos.1)))
    } else { None };

    // Left
    let left: Triple = grid.get(cur_pos.0)
        .and_then(|row| if cur_pos.1 > 0 { row.get(cur_pos.1-1) } else { None }
        .map(|char_| (char_, cur_pos.0, cur_pos.1-1)));
    // println!("?{}?\n{}0{}\n?{}?", 
    //     match up {
    //         Some(triple) => {triple.0},
    //         None => {&'x'},
    //     },
    //     match left {
    //         Some(triple) => {triple.0},
    //         None => {&'x'},
    //     },match right {
    //         Some(triple) => {triple.0},
    //         None => {&'x'},
    //     },match down {
    //         Some(triple) => {triple.0},
    //         None => {&'x'},
    //     },
    // );
    (up, right, down, left)
}

fn traverse_pipes(grid: &Grid, start_pos: (usize, usize)) -> Vec<(usize, usize)> {

    fn get_next_pos(direction: u8, surrounding_option: (&char, usize, usize), grid: &Vec<Vec<char>>, cur_pos: (usize, usize), old_pos: Option<(usize, usize)>, map: &DirectionMap) -> Option<(usize, usize)> {
        let (next_char, row, col) = surrounding_option;
        let next_pos = (row, col);
        let cur_char = grid.get(cur_pos.0).unwrap().get(cur_pos.1).unwrap();
        let coming_from_there = old_pos.is_some() && old_pos.unwrap().0 == next_pos.0 && old_pos.unwrap().1 == next_pos.1;
        let possible_connections = map.get(cur_char).unwrap().iter().filter_map(|(char_, direction_)| if *direction_ == direction {Some(char_)} else {None}).collect::<Vec<_>>();
        let symbol_connects = possible_connections.iter().find(|iter| **iter == next_char).is_some();
        //println!("coming_from_there: {:?} && symbol_connects: {:?}", coming_from_there, symbol_connects);
        if !coming_from_there && symbol_connects {
            return Some(next_pos);
        }
        None
    }

    type DirectionMap = HashMap<char, Vec<(char, u8)>>;
    let mut map = DirectionMap::new();
    map.insert('S', vec![
        ('|', 0), 
        ('|', 2), 
        ('J', 1),  
        ('J', 2),  
        ('L', 2), 
        ('L', 3), 
        ('7', 0), 
        ('7', 1), 
        ('F', 0), 
        ('F', 3), 
        ('-', 1), 
        ('-', 3), 
    ]);
    map.insert('|', vec![
        ('|', 2), 
        ('J', 2),  
        ('L', 2), 
        ('|', 0), 
        ('7', 0), 
        ('F', 0), 
    ]);
    map.insert('-', vec![
        ('-', 3), 
        ('F', 3), 
        ('L', 3),  
        ('-', 1), 
        ('7', 1), 
        ('J', 1),  
    ]);
    map.insert('F', vec![
        ('-', 1), 
        ('7', 1), 
        ('J', 1),  
        ('L', 2),  
        ('J', 2),  
        ('|', 2),  
    ]);
    map.insert('L', vec![
        ('|', 0),  
        ('F', 0),  
        ('7', 0),  
        ('-', 1),   
        ('7', 1),   
        ('J', 1),   
    ]);
    map.insert('J', vec![
        ('|', 0),  
        ('F', 0),  
        ('7', 0),  
        ('-', 3),   
        ('F', 3),   
        ('L', 3),   
    ]);
    map.insert('7', vec![
        ('|', 2),   
        ('J', 2),   
        ('L', 2),   
        ('-', 3),   
        ('L', 3),   
        ('F', 3),   
    ]);
    

    let mut path = Vec::new();
    let mut cur_pos = start_pos;
    
    loop {
        println!("now at {:?}", cur_pos);
        let (up,
            right,
            down,
            left
        ) = get_surroundings(grid, cur_pos);
        
        let surroundings = vec!(up, right, down, left);
        
        let old_pos: Option<(usize, usize)> = path.last().cloned();

        path.push(cur_pos);
        
        let mut found_next = false;
        for (idx, signature_option) in surroundings.into_iter().enumerate() {
            if let Some(signature) = signature_option {
                let next_pos = get_next_pos(idx as u8, signature, grid, cur_pos, old_pos.clone(), &map);
                if let Some(val) = next_pos {
                    let is_s = *grid.get(val.0).unwrap().get(val.1).unwrap() == 'S';
                    if is_s {
                        return path.clone(); // or just return "path"
                    } 
                    cur_pos = val;
                    found_next = true;
                    break;
                }
            } 
        }

        if !found_next { // no valid next position is found
            return path.clone();
        }
    }
}

pub fn p1() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    println!("grid: {:?}", grid);
    let s: (usize, usize) = find_s(&grid);
    println!("{:?}", s);
    let path = traverse_pipes(&grid, s);
    println!("{:?}", path);
    println!("{:?}", path.len());
    println!("result {:?}", (path.len() as f64 / 2.0).ceil());
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    println!("grid: {:?}", grid);
    let s: (usize, usize) = find_s(&grid);
    println!("{:?}", s);
    let path = traverse_pipes(&grid, s);
    println!("{:?}", path);
    println!("{:?}", path.len());

    let s_symbol: char = {
        let (up, right, down, left ) = get_surroundings(&grid, s);
        let up = up.is_some();
        let right = right.is_some();
        let down = down.is_some();
        let left = left.is_some();
        if up && down { '|' }
        else if up && right { 'L' }
        else if up && left { 'J' }
        else if right && left { '-' }
        else if down && left { '7' }
        else if down && right { 'F' }
        else { panic!("did not find a symbol for s") }
    };
    let mut grid = grid.clone();
    grid[s.0][s.1] = s_symbol;
    let grid = grid;

    let mut new_grid_from_horizontal: Vec<Vec<char>> = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        let mut cur_passed_path_segments = 0;
        for (col_idx, char_) in row.iter().enumerate() {
            let cur_pos_is_on_path = path.iter().find(|element| element.0 == row_idx && element.1 == col_idx).is_some();
            if cur_pos_is_on_path {
                let previous_char = row.get(col_idx-1).unwrap();
                // let connects_to_previous_symbol = *char_ != '-' 
                //     || *char_ == '7' &&  (*previous_char == '-' || *previous_char == 'L' || *previous_char == 'F')
                //     || *char_ == 'J' && (*previous_char == '-' || *previous_char == 'L' || *previous_char == 'F');
                // cur_passed_path_segments += if connects_to_previous_symbol { 0 } else { 1 };
                
                // let is_prev_symbol_counter = *previous_char == '.' || *previous_char == '|';
                // cur_passed_path_segments += if is_prev_symbol_counter { 1 } else { 0 };

                cur_passed_path_segments += 1;
                
                new_line.push(match char_ {
                    '7' => { '┓' },
                    'L' => { '┗' },
                    'J' => { '┛' },
                    'F' => { '┏' },
                    '|' => { '┃' },
                    '-' => { '━' },
                    _ => '?',
                });
            } else {
                if cur_passed_path_segments % 2 == 1 {
                    new_line.push('●');
                } else {
                    new_line.push(' ');
                }
            }
        }
        new_grid_from_horizontal.push(new_line);
    }

    let mut new_grid_from_vertical: Vec<Vec<char>> = Vec::new();
    for col_idx in 0..grid[0].len() {
        let mut new_line: Vec<char> = Vec::new();
        let mut cur_passed_path_segments = 0;
        for row_idx in 0..grid.len() {
            let char_ = &grid[row_idx][col_idx];
            let cur_pos_is_on_path = path.iter().find(|element| element.0 == row_idx && element.1 == col_idx).is_some();
            if cur_pos_is_on_path {
                //let previous_char = if row_idx > 0 { grid[row_idx - 1][col_idx] } else { '_' }; // Assume '_' for no previous character

                cur_passed_path_segments += 1;
                
                new_line.push(match char_ {
                    '7' => { '┓' },
                    'L' => { '┗' },
                    'J' => { '┛' },
                    'F' => { '┏' },
                    '|' => { '┃' },
                    '-' => { '━' },
                    _ => '?',
                });
            } else {
                if cur_passed_path_segments % 2 == 1 {
                    new_line.push('●');
                } else {
                    new_line.push(' ');
                }
            }
        }
        new_grid_from_vertical.push(new_line);
    }

    let mut final_grid: Vec<Vec<char>> = Vec::new();
    for row_idx in 0..new_grid_from_horizontal.len() {
        let mut new_line: Vec<char> = Vec::new();
        for col_idx in 0..new_grid_from_horizontal[0].len() {
            let from_horizontal = new_grid_from_horizontal[row_idx][col_idx];
            let from_vertical = new_grid_from_vertical[col_idx][row_idx];
            if from_horizontal == from_vertical {
                new_line.push(from_horizontal);
            } else {
                new_line.push(' ');
            }
        }
        final_grid.push(new_line);
    }
    let final_grid = final_grid;
    /*
    result:
    */

    let mut file: File = File::create("src/d10/grid.txt").expect("Could not create file");
    for row in &final_grid {
        let mut row_str: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        row_str.push("\n".to_string());
        let line = row_str.join("");
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }

    let result = final_grid.iter().fold(0, |sum, row| sum + row.iter().filter(|char_| **char_ == '●').collect::<Vec<_>>().len());
    println!("result: {result:?}{} ", if result >= 1391 { " which is too high" } else {""});
}