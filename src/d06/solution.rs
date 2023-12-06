use std::fs;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d06/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
        // Parse the lines into vectors of usize
        let time_line: Vec<usize> = input_lines[0].split(":").into_iter().collect::<Vec<_>>().get(1).unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let distance_line: Vec<usize> = input_lines[1].split(":").into_iter().collect::<Vec<_>>().get(1).unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", input_lines);

    // Zip the times and distances into races
    let races: Vec<(usize, usize)> = time_line.into_iter().zip(distance_line.into_iter()).collect();
    
    println!("{:?}", races);
    let result = get_product_of_len_of_ranges(races);
    println!("result {:?}", result);
}

fn get_product_of_len_of_ranges(races: Vec<(usize, usize)>) -> usize {
    races.iter().map(|(race_duration, record)| get_amount_of_full_seconds_push_winners(*race_duration, *record))
        .fold(1,|acc, iter| acc*iter)
}

fn get_amount_of_full_seconds_push_winners(race_duration: usize, record: usize) -> usize {
    let (root1, root2) = calculate_abc(-1.0, race_duration as f64, -(record as f64));
    (
        (root2-1.0).ceil()-
        (root1+1.0).floor()
        +1.0
    ) as usize
}

fn calculate_abc(a: f64, b: f64, c: f64) -> (f64, f64) {
    //println!("a: {a}, b: {b}, c: {c}");
    let determinant = b.powi(2) - 4.0 * a * c;
    //println!("determinant: {}", determinant);
    if determinant < 0.0 {
        panic!("No real roots exist for these values");
    } else {
        let root1 = (-b + determinant.sqrt() ) /(2.0 * a);
        let root2 = (-b - determinant.sqrt() )/ (2.0 * a);
        (root1, root2)
    }
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    // Parse the lines into vectors of usize
    let time: usize = input_lines[0].split(":").collect::<Vec<_>>()[1]
        .replace(" ", "").parse::<usize>().unwrap();
    let distance: usize = input_lines[1].split(":").collect::<Vec<_>>()[1]
        .replace(" ", "").parse::<usize>().unwrap();
    
    let result = get_amount_of_full_seconds_push_winners(time, distance);
    println!("result {:?}", result);
}
