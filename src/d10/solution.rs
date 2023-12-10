use std::{fs, collections::HashMap};

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


fn traverse_pipes(grid: &Grid, cur_pos: (usize, usize), path: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
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
    
    let old_pos: Option<(usize, usize)> = path.last().cloned();

    path.push(cur_pos);

    println!("now at {:?}", cur_pos);
    let (up,
        right,
        down,
        left
    ) = get_surroundings(grid, cur_pos);
    
    let surroundings = vec!(up, right, down, left);

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

    return surroundings.into_iter().enumerate().filter_map(|(idx, signature_option)| {
        signature_option.and_then(|signature| {
            let next_pos = get_next_pos(idx as u8, signature, grid, cur_pos, old_pos.clone(), &map);
            match next_pos {
                Some(val) => {
                    let is_s = *grid.get(val.0).unwrap().get(val.1).unwrap() == 'S';
                    if is_s {
                        return Some(path.clone());
                    } 
                    return Some(traverse_pipes(grid, val, path));
                },
                None => None,
            } 
        })
    }).next().unwrap_or(path.clone());
}

pub fn p1() {
    let puzzle_input = read_puzzle_input();
    let grid: Grid = puzzle_input.split("\n").map(|val| val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    println!("grid: {:?}", grid);
    let s: (usize, usize) = find_s(&grid);
    println!("{:?}", s);
    let path = &mut vec![];
    let path = traverse_pipes(&grid, s, path);
    println!("{:?}", path);
    println!("{:?}", path.len());
    println!("result {:?}", (path.len() as f64 / 2.0).ceil());
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let rows = puzzle_input.split("\n").map(|val|val.chars().into_iter().collect::<Vec<char>>()).collect::<Grid>();
    println!("grid: {:?}", rows);
}