use std::{fs::{self, File}, collections::HashSet, io::Write};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d16/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

type Grid = Vec<Vec<char>>;
type Coordinate = (usize, usize);
type Path = Vec<Coordinate>;

pub fn p1() {
    let _puzzle_input: String = read_puzzle_input();
    let grid: Vec<Vec<char>> = _puzzle_input
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<_>();
    let mut energized = HashSet::from([]);
    traverse(&grid, &mut energized, None, (0,0), Direction::Right);
    println!("result {:?}", deduplicate_coordinate_values_with_different_directions(energized).len());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction { Up, Right, Down, Left }

fn deduplicate_coordinate_values_with_different_directions(set: HashSet<(Coordinate, Direction)>) -> HashSet<(Coordinate, Direction)> {
    let mut first_values_set: HashSet<Coordinate> = HashSet::new();
    let mut result_set: HashSet<(Coordinate, Direction)> = HashSet::new();

    for tuple in set {
        if !first_values_set.contains(&tuple.0) {
            first_values_set.insert(tuple.0);
            result_set.insert(tuple);
        }
    }

    result_set
}

fn calculate_direction(from: Coordinate, to: Coordinate) -> Direction {
    if from.0 > to.0 {
        Direction::Up
    } else if from.0 < to.0 {
        Direction::Down
    } else if from.1 > to.1 {
        Direction::Left
    } else if from.1 < to.1 {
        Direction::Right
    } else {
        panic!("'from' and 'to' cannot be equal");
    }
}

fn get_next_coordinate((row, col): Coordinate, direction: Direction) -> Option<Coordinate> {
    if direction == Direction::Up && row == 0 { return None }
    if direction == Direction::Left && col == 0 { return None }
    match direction {
        Direction::Up => Some((row-1, col)),
        Direction::Right => Some((row, col+1)),
        Direction::Down => Some((row+1, col)),
        Direction::Left => Some((row, col-1)),
    }
}

fn valid_coordinate(grid: &Grid, coordinate: Coordinate) -> bool {
    get_value_of_grid(grid, coordinate).is_some()
}

fn get_value_of_grid(grid: &Grid, coordinate: Coordinate) -> Option<char> {
    grid.get(coordinate.0)?.get(coordinate.1).copied()
}

fn traverse(grid: &Grid, energized: &mut HashSet<(Coordinate, Direction)>, previous: Option<Coordinate>, cur: Coordinate, default_direction: Direction) {
    let direction_used_to_reach_cur = match previous {
        Some(previous) => {calculate_direction(previous, cur)},
        None => {Direction::Right},
    };
    energized.insert((cur, direction_used_to_reach_cur));
    //println!("working on coordinate: {:?}", cur);
    //let _write_result = write_energized_grid_to_file(&grid, energized);

    let cur_symbol = get_value_of_grid(grid, cur).expect("current coordinate is out of bounce");
    
    let directions = match cur_symbol {
        '.' => {
            handle_dot(previous, cur, default_direction.clone())
        },
        '/' => {
            handle_forward_slash(previous, cur, default_direction.clone())
        },
        '\\' => {
            handle_backward_slash(previous, cur, default_direction.clone())
        },
        '-' => {
            handle_minus(previous, cur, default_direction.clone())
        },
        '|' => {
            handle_pipe(previous, cur, default_direction.clone())
        },
        _char => {
            panic!("encountered unexpected character '{_char}'");
        }
    };

    let next_coordinates = directions.iter().map(|direction| get_next_coordinate(cur, direction.clone())).collect::<Vec<_>>();
    for next in next_coordinates.iter().filter_map(|x| *x) {
        let direction_used_to_reach_next_from_cur = calculate_direction(cur, next);
        if energized.contains(&(next, direction_used_to_reach_next_from_cur)) {
            return;
        }
        match valid_coordinate(grid, next) {
            true => {traverse(grid, energized, Some(cur), next, default_direction.clone());},
            false => {},
        }
    }
}

fn write_energized_grid_to_file(grid: &Grid, energized: &mut HashSet<(Coordinate, Direction)>) -> std::io::Result<()> {
    //println!("write path to file");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d16/energized_grid.txt");
    
    let mut file = File::create(file_path)?;
    for (i, row) in grid.iter().enumerate() {
        for (j, &char_) in row.iter().enumerate() {
            let char_to_write = if vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter().any(|direction| energized.contains(&((i,j), direction.clone()))) {'#'} else {char_};
            file.write_all(char_to_write.to_string().as_bytes())?;
        }
        file.write_all(b"\n")?;
    }
    //std::thread::sleep(std::time::Duration::from_millis(200));
    Ok(())
}

fn handle_dot(previous: Option<(usize, usize)>, cur: (usize, usize), default_direction: Direction) -> Vec<Direction> {
    let direction = match previous {
        Some(previous) => {calculate_direction(previous, cur)},
        None => {default_direction},
    };
    vec![direction]
}

fn handle_forward_slash(previous: Option<(usize, usize)>, cur: (usize, usize), default_direction: Direction) -> Vec<Direction> {
    let direction = match previous {
        Some(previous) => {
            match calculate_direction(previous, cur) {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            }
        },
        None => {
            match default_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
            }
        }
    };
    vec![direction]
}

fn handle_backward_slash(previous: Option<(usize, usize)>, cur: (usize, usize), default_direction: Direction) -> Vec<Direction> {
    let direction = match previous {
        Some(previous) => {
            match calculate_direction(previous, cur) {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            }
        },
        None => {
            match default_direction {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
            }
        },
    };
    vec![direction]
}

fn handle_minus(previous: Option<(usize, usize)>, cur: (usize, usize), default_direction: Direction) -> Vec<Direction> {
    match previous {
        Some(previous) => {
            match calculate_direction(previous, cur) {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Right => vec![Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
            }
        },
        None => {
            match default_direction {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Right => vec![Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Left],
            }
        },
    }
}

fn handle_pipe(previous: Option<(usize, usize)>, cur: (usize, usize), default_direction: Direction) -> Vec<Direction> {
    match previous {
        Some(previous) => {
            match calculate_direction(previous, cur) {
                Direction::Up => vec![Direction::Up],
                Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
            }
        },
        None => {
            match default_direction {
                Direction::Up => vec![Direction::Up],
                Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
            }
        },
    }
}

pub fn p2() {
    let _puzzle_input: String = read_puzzle_input();
    let grid: Vec<Vec<char>> = _puzzle_input
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<_>();
    let height = grid.len();
    let width = grid.get(0).unwrap().len();
    let mut values: Vec<usize> = vec![];
    for y in 0..height {
        let start = (y, 0);
        let default_direction = Direction::Right;
        let mut energized = HashSet::from([]);
        traverse(&grid, &mut energized, None, start, default_direction);
        let value = deduplicate_coordinate_values_with_different_directions(energized).len();
        values.push(value);
    }
    for y in 0..height {
        let start = (y, width-1);
        let default_direction = Direction::Left;
        let mut energized = HashSet::from([]);
        traverse(&grid, &mut energized, None, start, default_direction);
        let value = deduplicate_coordinate_values_with_different_directions(energized).len();
        values.push(value);
    }
    for x in 0..width {
        let start = (0, x);
        let default_direction = Direction::Down;
        let mut energized = HashSet::from([]);
        traverse(&grid, &mut energized, None, start, default_direction);
        let value = deduplicate_coordinate_values_with_different_directions(energized).len();
        values.push(value);
    }
    for x in 0..width {
        let start = (height-1, x);
        let default_direction = Direction::Up;
        let mut energized = HashSet::from([]);
        traverse(&grid, &mut energized, None, start, default_direction);
        let value = deduplicate_coordinate_values_with_different_directions(energized).len();
        values.push(value);
    }
    println!("result {:?}", values.iter().max());
}