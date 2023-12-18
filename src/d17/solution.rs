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
   
    println!("starting at (0,0)");
    let opt_path = traverse(&grid, vec![(0,0)], usize::MAX).expect("did not find any path");
    println!("resulting optimal path: {:?}", opt_path);
    println!("costs: {}", get_value_of_path(&grid, &opt_path));
}

type Grid = Vec<Vec<u8>>;
type Coordinate = (u8, u8);
type Path = Vec<Coordinate>;
fn traverse(grid: &Grid, path: Path, cur_best_score: usize) -> Option<Vec<(u8, u8)>> {
    //println!("--recursion--");
    //println!("current path length: {}", path.len());
    // init
    let cur_coordinate = path.last().unwrap();
    let coming_from: Option<Coordinate> = path.iter().rev().take(2).last().copied();
    let grid_width = grid.len();
    let grid_height = grid.get(0).unwrap().len();
    let destination = ((grid_height-1) as u8, (grid_width-1) as u8);
    //println!("searching destination: {:?} from {:?}", destination, cur_coordinate);
   
    // early stopping of worse paths
    let value_of_current_path = get_value_of_path(&grid, &path);
    let min_manhattan_distance_to_goal = (grid_height - cur_coordinate.0 as usize) + (grid_width - cur_coordinate.1 as usize);
    //println!("{cur_coordinate:?}: {min_manhattan_distance_to_goal}");
    if (value_of_current_path + min_manhattan_distance_to_goal) >= cur_best_score {
        //println!("early stopping this path because it cannot get better than the current best path either way");
        return None;
    }
    // anchor
    if *cur_coordinate == destination {
        //println!("found path reaching destination {:?}", path);
        return Some(path);
    }
    
    // recursion (split cur point in up to 3 new ones):
    
    // calculate all the possible options to go to using get_adjacent_coordinates
    let adjacent_coordinates = get_adjacent_coordinates(&grid, *cur_coordinate);
    // remove the ones that are not in the grid anyways (option => None) (negative col or row or too far down or right)
    let possible_coordinates = adjacent_coordinates.iter()
        .filter_map(|coordinate| *coordinate)
        // remove the one that would result in 4 steps into the same direction (last 4 elements all in the same row or all in same col)
        .filter(|coordinate| !all_same_row_or_col(&path, *coordinate))
        // remove already visited nodes because circles dont make sense
        .filter(|coordinate| {
            path.iter().find(|prior_coordinate| *prior_coordinate == coordinate).is_none()
        });
        
    // println!("now on {:?}", cur_coordinate);
    // println!("came from {:?}", coming_from);
    // println!("{} potential points to go to from here", possible_coordinates.clone().collect::<Vec<_>>().len());
    // println!("{:?}", possible_coordinates.clone().collect::<Vec<_>>());
    
    // for each remaining possible next point start the depth search in recursion
    let mut recursion_results: Vec<Path> = vec![];
    let mut cur_best_score = cur_best_score;
    for next_coordinate in possible_coordinates {
        let mut next_path = path.clone();
        next_path.push(next_coordinate);
        let path_option = traverse(&grid, next_path.clone(), cur_best_score);
        //println!("path option for this possible coordinate: {path_option:?}");
        match path_option {
            Some(new_opt_path) => {
                // if any iteration returns something it means that it found a path better than the cur_best_score
                // => update local cur_best_score and push the path to the recursion_results
                let new_opt = get_value_of_path(&grid, &new_opt_path);
                if new_opt < cur_best_score {
                    cur_best_score = new_opt;
                    //println!("found a path with better score of {} with lenght {}\nwriting path to file", cur_best_score, new_opt_path.len());
                    //println!("{:?}", new_opt_path);
                    let _write_res = write_grid_with_path_to_file(&grid, &new_opt_path);
                    recursion_results.push(new_opt_path);
                }
            },
            None => {},
        };
    }
    // collect the results and return the last Some(path) in the list (every after another indicates that the old one was worse - otherwise it would be None)
    //println!("all paths resutling from the current coordinate:\n{recursion_results:?}");
    if recursion_results.len() > 0 {
        println!("current best score: {}", cur_best_score);
        println!("scores: {:?}", recursion_results.iter().map(|val| get_value_of_path(&grid, &val)).collect::<Vec<_>>());
    }
    match recursion_results.last() {
        Some(best_path) => {
            if *best_path.last().unwrap() != destination {
                panic!("Path-Result is Some() although last element is not destination\nfound: {:?} expected: {:?}", *best_path.last().unwrap(), destination);
            }
            if best_path.len() < (grid_height + grid_width) {
                panic!("returned a path smaller than the minimal length possible");
            }
            return Some(best_path.clone());
        },
        None => {return None;},
    }
}

fn all_same_row_or_col(path: &Vec<(u8, u8)>, coordinate: (u8, u8)) -> bool{
    let last_coordinates = path.iter().rev().take(4);
    if last_coordinates.len() < 4 {return false}
    let all_same_row = last_coordinates.clone().all(|prior_coordinate| prior_coordinate.0 == coordinate.0);
    let all_same_col = last_coordinates.clone().all(|prior_coordinate| prior_coordinate.1 == coordinate.1);
    //println!("{:?} and {:?} have {}same rows and {}same cols", last_coordinates.collect::<Vec<_>>(), coordinate, if all_same_row {""} else {"not "}, if all_same_col {""} else {"not "});
    all_same_col || all_same_row
}
#[cfg(test)]
mod merge_chunks_tests {
    use super::*;

    #[test]
    fn all_same_row_or_col_test() {
        let result = all_same_row_or_col(&vec![(0,0)], (0,1));
        assert_eq!(result, true);
        
        let result = all_same_row_or_col(&vec![(0,0), (0,1)], (0,2));
        assert_eq!(result, true);

        let result = all_same_row_or_col(&vec![(0,0), (1,0)], (2,0));
        assert_eq!(result, true);

        let result = all_same_row_or_col(&vec![(0,0), (1,0), (2,0), (3,0), (3,1)], (3,2));
        assert_eq!(result, false);

        let result = all_same_row_or_col(&vec![(0,0), (1,0), (2,0), (3,0), (3,1)], (4,2));
        assert_eq!(result, false);

        let result = all_same_row_or_col(&vec![(1,1), (1,0), (2,0), (3,0), (4,0)], (5,0));
        assert_eq!(result, true);

        let result = all_same_row_or_col(&vec![(1,1), (1,0), (2,0), (3,0), (4,0)], (4,1));
        assert_eq!(result, false);

        let result = all_same_row_or_col(&vec![(0,0), (1,0), (1,1)], (1,2));
        assert_eq!(result, false);
    }
}

fn get_value_of_path(grid: &Vec<Vec<u8>>, path: &Vec<(u8,u8)>) -> usize {
    path.iter().fold(0, |acc, coordinate| {
        let value_at_coord = *grid.get(coordinate.0 as usize).unwrap().get(coordinate.1 as usize).unwrap() as usize;
        acc + value_at_coord
    })
}

fn write_grid_with_path_to_file(grid: &Vec<Vec<u8>>, path: &[(u8, u8)]) -> std::io::Result<()> {
    println!("write path to file");
    // timeout to inspect the grid.
    //std::thread::sleep(std::time::Duration::from_millis(200));
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