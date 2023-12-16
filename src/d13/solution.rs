use std::{fs::{self}, collections::{HashMap, HashSet}};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d13/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mirror_col() {

        let grid = vec![
            "....###.##..".to_string(),
            "####.#...#..".to_string(),
            "#..##.#...#.".to_string(),
            "#..#.#.#..##".to_string(),
            "....#.....##".to_string()
        ];

        assert_eq!(is_mirror_col(&grid, 0), false);
        assert_eq!(is_mirror_col(&grid, 1), true);
        assert_eq!(is_mirror_col(&grid, 10), false);
        assert_eq!(is_mirror_col(&grid, 100), false);
    }

    #[test]
    fn example_grid_1_test() {

        let grid = vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string()
        ];
        for col in 0..11 {
            assert_eq!(is_mirror_col(&grid, col), if col == 5 {true} else {false});
        }
        for row in 0..17 {
            assert_eq!(is_mirror_row(&grid, row), false);
        }
    }

    
    #[test]
    fn example_grid_7_test() {

        let grid = vec![
            "#......#....##.".to_string(),
            "####.###.#...#.".to_string(),
            "####.###.#...#.".to_string(),
            "#......#....##.".to_string(),
            "..##...#...#...".to_string(),
            "..#...#####.#..".to_string(),
            "#####.#####.###".to_string(),
            "...####...##.#.".to_string(),
            "...####...##.#.".to_string()
        ];
        for col in 0..20 {
            assert_eq!(is_mirror_col(&grid, col), false);
        }
        assert_eq!(is_mirror_row(&grid, 7), true);
        for row in 0..11 {
            assert_eq!(is_mirror_row(&grid, row), if row == 1 || row == 7 {true} else {false});
        }

        let permutations = bit_flip_permutations(&grid);
        permutations.iter().enumerate()
            .for_each(|(idx, permutation)| if idx == 15*9 - 1 {
                    permutation.iter().for_each(|line| println!("{:?}", line));
            });
        assert_eq!(permutations.len(), 15*9);
    }   
    
    #[test]
    fn example_grid_permutation() {

        let grid = vec![
            "#......#....##.".to_string(),
            "####.###.#...#.".to_string(),
            "####.###.#...#.".to_string(),
            "#......#....##.".to_string(),
            "..##...#...#...".to_string(),
            "..#...#####.#..".to_string(),
            "#####.#####.###".to_string(),
            "...####...##.#.".to_string(),
            "...####...#..#.".to_string()
        ];

        let permutations = bit_flip_permutations(&grid);
        permutations.iter().enumerate()
            .for_each(|(idx, permutation)| if idx == 15*9 - 4 {
                    permutation.iter().for_each(|line| println!("{:?}", line));
            });
        assert_eq!(permutations.len(), 15*9);
    }

    #[test]
    fn test_is_mirror_row() {

        let grid = vec![
            "...###.##..".to_string(),
            "###.#...#..".to_string(),
            "###.#...#..".to_string(),
            "...###.##..".to_string(),
            "..#.#.#..##".to_string(),
            "...#.....##".to_string()
        ];

        assert_eq!(is_mirror_row(&grid, 0), false);
        assert_eq!(is_mirror_row(&grid, 1), true);
        assert_eq!(is_mirror_row(&grid, 2), false);
        assert_eq!(is_mirror_row(&grid, 3), false);
        assert_eq!(is_mirror_row(&grid, 4), false);
        assert_eq!(is_mirror_row(&grid, 5), false);
        assert_eq!(is_mirror_row(&grid, 10), false);
    }

    #[test]
    fn test_is_mirror_row_2() {

        let grid = vec![
            "##.#..#".to_string(),
            "###....".to_string(),
            "#..####".to_string(),
            ".######".to_string(),
            "#.##..#".to_string(),
            "###....".to_string(),
            "##..##.".to_string(),
            "..#####".to_string(),
            "##.####".to_string(),
            "####..#".to_string(),
            "#.#####".to_string(),
            "#.#####".to_string(),
            "####..#".to_string(),
            "##.####".to_string(),
            "..#####".to_string(),
            "##..##.".to_string(),
            "##.....".to_string()
        ];

        assert_eq!(is_mirror_col(&grid, 0), false);
        assert_eq!(is_mirror_col(&grid, 1), false);
        assert_eq!(is_mirror_col(&grid, 2), false);
        assert_eq!(is_mirror_col(&grid, 3), false);
        assert_eq!(is_mirror_col(&grid, 4), true);
        assert_eq!(is_mirror_col(&grid, 5), false);
        assert_eq!(is_mirror_col(&grid, 6), false);
        assert_eq!(is_mirror_col(&grid, 7), false);
    }
}

fn is_mirror_col(grid: &Vec<String>, col_idx: usize) -> bool {
    let n_cols = grid[0].len();

    if col_idx >= (n_cols - 1) {
        return false;
    }
    
    let mut left_idx = col_idx;
    let mut right_idx = col_idx + 1;
    //println!("{}-{}", left_idx, right_idx);
    for row in grid {
        if row.chars().nth(left_idx) != row.chars().nth(right_idx) {
            return false;
        }
    }
    //println!("{}-{} same, increasing size", left_idx, right_idx);
    
    while right_idx < n_cols  {        
        //println!("{}-{}", left_idx, right_idx);
        for row in grid {
            if row.chars().nth(left_idx) != row.chars().nth(right_idx) {
                //println!("{}-{} not same same", left_idx, right_idx);
                //println!("{:?}-{:?} not same same", row.chars().nth(left_idx), row.chars().nth(right_idx));
                return false;
            }
        }
        //println!("{}-{} same, increasing size", left_idx, right_idx);
        left_idx = match left_idx.checked_sub(1) {
            Some(val) => {val},
            None => {break},
        };
        right_idx += 1;
    }
    
    true
}

fn is_mirror_row(grid: &Vec<String>, row_idx: usize) -> bool {
    let n_rows = grid.len();
    
    if row_idx >= (n_rows - 1) {
        return false;
    }
    
    let mut up_idx = row_idx;
    let mut down_idx = row_idx + 1;
    
    if grid[up_idx] != grid[down_idx] {
        return false;
    }

    while down_idx < n_rows {
        if grid[up_idx] != grid[down_idx] {
            return false;
        }
        up_idx = match up_idx.checked_sub(1) {
            Some(val) => {val},
            None => {break},
        };
        down_idx += 1;
    }
    true
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let grids: Vec<Vec<String>> = puzzle_input.split("\n\n").map(|grid| {
        grid.split("\n")
            .map(|x | x.to_string())
            .collect::<Vec<String>>()
    }).collect::<Vec<Vec<_>>>();
    
    // grids.iter().for_each(|chunk| {
    //     println!("");
    //     chunk.iter().for_each(|line| println!("{:?}", line))
    // });

    let grid_mirrors = grids.iter()
        .map(|grid| {
            let rows = grid.len();
            for row in 0..rows {
                if is_mirror_row(grid, row) { return (0, row+1)}
            }
            let cols = grid.get(0).unwrap().len();
            for col in 0..cols {
                if is_mirror_col(grid, col) { return (1, col+1) }
            }
            //grid.iter().for_each(|line| println!("{:?}", line));
            panic!("did not find any mirror row or col");

        })
        .collect::<Vec<_>>();
    let sum: usize = grid_mirrors.iter().fold(0, |acc, (type_, val)| acc + (if *type_ == 0 { 100 * val } else { *val }) );
    //grid_mirrors.iter().for_each(|grid_solution| println!("{} {} mirrors the grid", if grid_solution.0 == 0 {"row"} else {"col"}, grid_solution.1));
    println!("{:#?} {}", sum, if sum <= 29421 { "which is too low"} else {""});
}

fn bit_flip_permutations(grid: &Vec<String>) -> Vec<Vec<String>> {
    let mut results = Vec::new();
    
    for (i, row) in grid.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        for (j, c) in chars.iter().enumerate() {
            let mut grid_copy = grid.clone();
            let mut row_copy: Vec<char> = grid_copy[i].chars().collect();
            row_copy[j] = match c {
                '#' => '.',
                '.' => '#',
                _ => panic!("Invalid character"),
            };
            let new_grid = &row_copy.iter().collect::<String>();
            grid_copy[i] = new_grid.to_string();
            let grid_copy: Vec<String> = grid_copy.iter().map(|s| s.chars().collect()).collect();
            results.push(grid_copy);
        }
    }
    
    results
}

fn retain_uniques(vec: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut counts = HashMap::new();
    for tup in vec.iter() {
        *counts.entry(*tup).or_insert(0) += 1;
    }

    vec.into_iter()
        .filter(|tup| counts[tup] == 2)
        .collect::<HashSet<_>>() // We use HashSet to eliminate duplicates
        .into_iter()
        .collect()
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
    let grids = puzzle_input.split("\n\n").map(|grid| {
        grid.split("\n")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    }).collect::<Vec<Vec<_>>>();
    let grids_with_permutations = grids.iter()
        .map(|original_grid| (original_grid, bit_flip_permutations(original_grid)));

    let grids_with_correct_permutation = grids_with_permutations.map(|(original_grid, permutations_of_gird)| {
        let original_grid_result: (usize, usize) = {
            let rows = original_grid.len();
            let mut result = None;
            for row in 0..rows {
                if is_mirror_row(original_grid, row) { 
                    result = Some((0, row+1)); 
                    break;
                }
            }
            if result.is_none() {
                let cols = original_grid.get(0).unwrap().len();
                for col in 0..cols {
                    if is_mirror_col(original_grid, col) { 
                        result = Some((1, col+1)); 
                        break;
                    }
                }
            }
            match result {
                Some(res) => res,
                None => panic!("did not find original result"),
            }
        };
        let permutations_with_possible_mirror = permutations_of_gird.iter().enumerate().filter_map(|(permutation_idx, permutation)| {
            let rows = permutation.len();
            let cols = permutation.get(0).unwrap().len();
            //println!("\nchecking a total of {rows} and {cols} for a position for the mirror differing from {:?}\n", original_grid_result);
            for row in 0..rows {
                if row == original_grid_result.1 - 1 && original_grid_result.0 == 0 { 
                    // println!("not checking row {row:?} - same row of original result");
                    continue
                }
                // println!("checking row {row:?} for permutation: {permutation_idx} in grid {grid_idx}");
                // this should not early return if the first mirror is found because it could be the old mirror pos
                let is_mirrorable_at_pos = is_mirror_row(permutation, row);
                if is_mirrorable_at_pos { 
                    // println!("found mirrorable position at row {}", row);
                    return Some((permutation_idx, (0, row+1)))
                }
            }
            for col in 0..cols {
                if col == original_grid_result.1 - 1 && original_grid_result.0 == 1 { 
                    // println!("not checking col {col:?} - same col of original result");
                    continue
                }
                // println!("checking col {col:?} for permutation: {permutation_idx} in grid {grid_idx}");
                // this should not early return if the first mirror is found because it could be the old mirror pos
                let is_mirrorable_pos = is_mirror_col(permutation, col);
                if is_mirrorable_pos {
                    // println!("found mirrorable position at col {}", col);
                    return Some((permutation_idx, (1, col+1)))
                }
            }
            // println!("permutation: {permutation_idx} in grid {grid_idx} has no new mirrorable position");
            return None;
        })
        .collect::<Vec<_>>();
        // println!("mirror_position for permutations with a result {:?}", permutations_with_possible_mirror);
        let permutations_with_possible_mirror_and_no_other_same_solution = retain_uniques(permutations_with_possible_mirror.iter().map(|(_permutation_idx, result)| *result).collect::<Vec<_>>());
        // println!("{:?}", permutations_with_possible_mirror_and_no_other_same_solution);
        assert_eq!(permutations_with_possible_mirror_and_no_other_same_solution.len(), 1);
        // todo: doesnt find the permutation where row_idx 8 and col_idx 11 flips from . to #;
        let correct_permutation = permutations_with_possible_mirror_and_no_other_same_solution.get(0).unwrap();
        *correct_permutation
    })
    .collect::<Vec<(usize, usize)>>();

    // grids_with_correct_permutation.iter().enumerate().for_each(|(idx, grid_solution)| 
    //     println!("{idx}: {} {} mirrors the grid", if grid_solution.0 == 0 {"row"} else {"col"}, grid_solution.1)
    // );
    let sum: usize = grids_with_correct_permutation.iter().fold(0, |acc, (type_, val)| acc + (if *type_ == 0 { 100 * val } else { *val }) );
    println!("result after fixing exactly one smudge:\n{:#?}", sum);

}