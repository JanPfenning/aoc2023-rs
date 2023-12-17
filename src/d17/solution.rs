use std::fs;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d17/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let _puzzle_input: String = read_puzzle_input();
    let grid: Vec<Vec<u8>> = _puzzle_input
        .split('\n')
        .map(|row| row.chars().map(|char_| (char_ as u8)).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    print_grid(&grid);
    traverse(&grid, (0,0), vec![])
}

fn traverse(grid: &Vec<Vec<u8>>, at: (u8,u8), path: Vec<(u8,u8)>){
    //println!("{at:?}");
    let adjacent_coordinates = get_adjacent_coordinates(&grid, at);
    if at == ((grid.len()-1) as u8, (grid.get(0).unwrap().len()-1) as u8) {
        println!("{path:?}");
        println!("found end");
    }
    for coordinate in adjacent_coordinates.iter().filter_map(|val| if val.is_some() {Some(val)} else {None}) {
        //println!("checking whether to go to {coordinate:?}");
        let already_visited = path.clone().contains(&coordinate.unwrap());
        if already_visited {
            //println!("already visited");
            continue
        }
        let last_three_coordinates = path.iter().rev().take(3).collect::<Vec<_>>();
        let same_row_in_last_three_turns = last_three_coordinates.iter().all(|previous_direction| previous_direction.0 == last_three_coordinates.get(0).unwrap().0);
        let same_col_in_last_three_turns = last_three_coordinates.iter().all(|previous_direction| previous_direction.1 == last_three_coordinates.get(0).unwrap().1);
        let same_directions_in_last_three_turns = same_row_in_last_three_turns || same_col_in_last_three_turns;
        if last_three_coordinates.len() == 3 && same_directions_in_last_three_turns {
            //println!("cant go 3 times into the same direction");
            continue
        }
        let mut new_path = path.clone();
        new_path.push(coordinate.unwrap());
        //if path.len() > 160 {println!("{}", path.len())};
        traverse(grid, coordinate.unwrap(), new_path)
    }
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    grid.iter()
        .for_each(|row| 
            println!("{}", row.iter().map(|char_| (*char_ as char).to_string()).collect::<Vec<String>>().join(""))
        );
}

fn get_adjacent_coordinates(grid: &Vec<Vec<u8>>, (row, col): (u8, u8)) -> [Option<(u8,u8)>; 4] {
    let mut adjacent_coordinates: [Option<(u8,u8)>; 4] = [None; 4];
    if row>0 {
        adjacent_coordinates[0] = Some((row - 1, col));
    }
    if (col as usize) < (grid.get(0).unwrap().len()-1) {
        adjacent_coordinates[1] = Some((row, col + 1));
    }
    if (row as usize) < (grid.len()-1) {
        adjacent_coordinates[2] = Some((row + 1, col));
    }
    if col>0 {
        adjacent_coordinates[3] = Some((row, col - 1));
    }
    adjacent_coordinates
}

pub fn p2() {
    let _puzzle_input = read_puzzle_input();
}