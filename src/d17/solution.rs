use std::{fs::{self, File}, io::Write};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d17/test_input.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let _puzzle_input: String = read_puzzle_input();
    let grid: Vec<Vec<u8>> = _puzzle_input
        .split('\n')
        .map(|row| row.chars().map(|char_| (char_ as u8 - 48)).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    print_grid(&grid);
    println!("value {}",  *grid.get(0 as usize).unwrap().get(0 as usize).unwrap() as usize);
   
    println!("starting at (0,0)");
    traverse(&grid, (0,0), vec![(0,0)], usize::MAX);
}

fn get_value_of_path(grid: &Vec<Vec<u8>>, path: &Vec<(u8,u8)>) -> usize {
    path.iter().fold(0, |acc, coordinate| {
        let value_at_coord = *grid.get(coordinate.0 as usize).unwrap().get(coordinate.1 as usize).unwrap() as usize;
        acc + value_at_coord
    })
}

fn traverse(grid: &Vec<Vec<u8>>, at: (u8,u8), path: Vec<(u8,u8)>, cur_opt: usize) -> Option<Vec<(u8,u8)>>{
    //println!("{at:?}");
    let adjacent_coordinates = get_adjacent_coordinates(&grid, at);
    if at == ((grid.len()-1) as u8, (grid.get(0).unwrap().len()-1) as u8) {
        //println!("found new cur best path end");
        let _file_write_res = write_grid_with_path_to_file(&grid, &path);
        return Some(path);
    }
    let mut adjacent_coordinates = adjacent_coordinates;
    adjacent_coordinates.sort_by(
        |a,b| {
            let a_val = if a.is_some() {
                *grid.get(a.unwrap().0 as usize).unwrap().get(a.unwrap().1 as usize).unwrap() as u8
            } else { u8::MAX };
            let b_val = if b.is_some() {
                *grid.get(b.unwrap().0 as usize).unwrap().get(b.unwrap().1 as usize).unwrap() as u8
            } else { u8::MAX };
            a_val.cmp(&b_val)
        }
    );
    let adjacent_coordinates: [Option<(u8, u8)>; 4] = adjacent_coordinates;
    let mut paths = vec![];
    let mut cur_opt = cur_opt;
    for coordinate in adjacent_coordinates.iter().filter_map(|x| *x) {
        //println!("checking whether to go to {coordinate:?}");
        let already_visited = path.clone().contains(&coordinate);
        if already_visited {
            //println!("already visited");
            continue
        }
        let last_three_coordinates = path.iter().rev().take(5).collect::<Vec<_>>();
        let same_row_in_last_three_turns = last_three_coordinates.iter().all(|previous_direction| previous_direction.0 == last_three_coordinates.get(0).unwrap().0);
        let same_col_in_last_three_turns = last_three_coordinates.iter().all(|previous_direction| previous_direction.1 == last_three_coordinates.get(0).unwrap().1);
        let same_directions_in_last_three_turns = same_row_in_last_three_turns || same_col_in_last_three_turns;
        if last_three_coordinates.len() == 5 && same_directions_in_last_three_turns {
            //println!("cant go 3 times into the same direction");
            continue
        }
        let mut new_path = path.clone();
        new_path.push(coordinate);
        let cur_val = get_value_of_path(&grid, &new_path);
        
        if cur_val >= cur_opt {
            continue 
        }
        let p = traverse(grid, coordinate, new_path, cur_opt);
        match p {
            Some(p) => {
                cur_opt = get_value_of_path(&grid, &p);
                println!("new cur_opt = {cur_opt} for path with len {}", p.len());
                paths.push(p)
            },
            None => {},
        }
    }
    let better_path = paths.last().and_then(|x| Some(x.to_owned())).or_else(|| None);
    return better_path;
}

fn write_grid_with_path_to_file(grid: &Vec<Vec<u8>>, path: &[(u8, u8)]) -> std::io::Result<()> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d17/test_input_path.txt");
    
    let mut file = File::create(file_path)?;
    for (i, row) in grid.iter().enumerate() {
        for (j, &digit) in row.iter().enumerate() {
            let mut char_to_write = (digit + 48) as char;
            if path.contains(&(i as u8, j as u8)) {
                let (prev_i, prev_j) = path[path.iter().position(|&x| x == (i as u8, j as u8)).unwrap_or(0).saturating_sub(1)];
                if i > 0 && prev_i == i as u8 - 1 {
                    char_to_write = 'v';
                } else if prev_i == i as u8 + 1 {
                    char_to_write = '^';
                } else if j > 0 && prev_j == j as u8 - 1 {
                    char_to_write = '>';
                } else if prev_j == j as u8 + 1 {
                    char_to_write = '<';
                } else {
                    char_to_write = '#';
                }
            }
            file.write_all(char_to_write.to_string().as_bytes())?;
        }
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    grid.iter()
        .for_each(|row: &Vec<u8>| 
            println!("{}", row.iter().map(|char_: &u8| ((*char_  + 48) as char).to_string()).collect::<Vec<String>>().join(""))
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