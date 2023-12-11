use std::{fs::{self, File}, collections::{HashMap, VecDeque}, io::Write};

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
        //println!("now at {:?}", cur_pos);
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



fn pad_matrix(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix = Vec::new();

    for (_row_idx, row) in matrix.iter().enumerate() {
        let mut new_row = Vec::new();
        for (_col_idx, &item) in row.iter().enumerate() {
            new_row.push(item);
            new_row.push('x');
        }
        new_matrix.push(new_row.clone());
        new_matrix.push(new_row.iter().enumerate().map(|(_col_idx, _char)| 'x').collect::<Vec<_>>());
    }

    for row_idx in 0..new_matrix.len() {
        for col_idx in 0..new_matrix[row_idx].len() {
            if new_matrix[row_idx][col_idx] == 'x' {
                new_matrix[row_idx][col_idx] = get_filler_ascii_char(&new_matrix, (row_idx, col_idx));
            }
        }
    }

    new_matrix
}

fn get_filler_ascii_char(matrix: &Vec<Vec<char>>, (row_idx, col_idx): (usize, usize)) -> char {
    let (up, right, down, left) = get_surroundings(&matrix, (row_idx, col_idx));
    let up_connects = up.is_some() && (*up.unwrap().0 == '┃' || *up.unwrap().0 == '┏' || *up.unwrap().0 == '┓');
    let right_connects = right.is_some() && (*right.unwrap().0 == '━' || *right.unwrap().0 == '┛' || *right.unwrap().0 == '┓');
    let down_connects = down.is_some() && (*down.unwrap().0 == '┃' || *down.unwrap().0 == '┛' || *down.unwrap().0 == '┗');
    let left_connects = left.is_some() && (*left.unwrap().0 == '━' || *left.unwrap().0 == '┏' || *left.unwrap().0 == '┗');
    let char_ = if up_connects && down_connects {'┃'} else if right_connects && left_connects { '━' } else {' '};
    char_
}

fn get_s(matrix: &Vec<Vec<char>>, (row_idx, col_idx): (usize, usize)) -> char {
    let (up, right, down, left) = get_surroundings(&matrix, (row_idx, col_idx));
    let up_connects = up.is_some() && (*up.unwrap().0 == '|' || *up.unwrap().0 == 'F' || *up.unwrap().0 == '7');
    let right_connects = right.is_some() && (*right.unwrap().0 == '━' || *right.unwrap().0 == 'J' || *right.unwrap().0 == '7');
    let down_connects = down.is_some() && (*down.unwrap().0 == '|' || *down.unwrap().0 == 'J' || *down.unwrap().0 == 'L');
    let left_connects = left.is_some() && (*left.unwrap().0 == '━' || *left.unwrap().0 == 'F' || *left.unwrap().0 == 'L');
    let char_ = if up_connects && right_connects {'L'}
         else if up_connects && down_connects { '|' } 
         else if up_connects && left_connects { 'J' } 
         else if right_connects && down_connects { 'F' } 
         else if right_connects && left_connects { '-' } 
         else if down_connects && left_connects { '7' } 
         else {' '};
    char_
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    let s: (usize, usize) = find_s(&grid);
    println!("s: {:?}", s);
    let path = traverse_pipes(&grid, s);

    let s_symbol: char = get_s(&grid, s);
    let mut grid = grid.clone();
    grid[s.0][s.1] = s_symbol;
    let grid = grid;
    
    let mut ascii_grid: Vec<Vec<char>> = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        let mut new_line: Vec<char> = Vec::new();
        for (col_idx, char_) in row.iter().enumerate() {
            let cur_pos_is_on_path = path.iter().find(|element| element.0 == row_idx && element.1 == col_idx).is_some();
            if cur_pos_is_on_path {
                new_line.push(match char_ {
                    '7' => { '┓' },
                    'L' => { '┗' },
                    'J' => { '┛' },
                    'F' => { '┏' },
                    '|' => { '┃' },
                    '-' => { '━' },
                    _ => '?',
                });
            }else{
                new_line.push(' ');
            }
        }
        ascii_grid.push(new_line);
    }

    let mut padded_grid: Vec<Vec<char>> = pad_matrix(&ascii_grid);
    flood_fill(&mut padded_grid, (150, 140), '●');
    
    let mut file: File = File::create("src/d10/grid.txt").expect("Could not create file");
    for row in padded_grid.clone() {
        let mut row_str: Vec<String> = row.iter().map(|c| c.to_string()).collect();
        row_str.push("\n".to_string());
        let line = row_str.join("");
        file.write_all(line.as_bytes()).expect("Could not write to file");
    }

    let final_grid: Vec<Vec<char>> = padded_grid.into_iter().enumerate()
        .filter(|&(i, _)| i % 2 == 0).map(|(_, row)| 
            row.into_iter().enumerate().filter(|&(j, _)| j % 2 == 0).map(|(_, c)| c).collect()
        ).collect();

    let result = final_grid.iter().fold(0, |sum, row| sum + row.iter().filter(|char_| **char_ == '●').collect::<Vec<_>>().len());
    println!("result: {result:?}{} ", if result >= 1391 { " which is too high" } else {""});
}

fn flood_fill(grid: &mut Vec<Vec<char>>, (x, y): (usize, usize), new_char: char) {
    let old_char = ' ';  // Assuming we're filling space.
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