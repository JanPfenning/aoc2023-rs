use std::fs::{self};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d12/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let mut split = line.split_whitespace();
    let first = split.next().unwrap().to_owned();
    let first = std::iter::repeat(first).take(5).collect::<Vec<String>>().join("?");
    let second = split.next().unwrap();
    let second = std::iter::repeat(second).take(5)
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .join(",");
    let second = second.split(',').into_iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line1 = ".# 1";
        let line2 = "???.### 1,1,3";

        assert_eq!((String::from(".#?.#?.#?.#?.#"), vec![1,1,1,1,1]), parse_line(line1));
        assert_eq!((String::from("???.###????.###????.###????.###????.###"), vec![1,1,3,1,1,3,1,1,3,1,1,3,1,1,3]), parse_line(line2));
    }
}

pub fn p2() {
    let puzzle_input: String = read_puzzle_input();
    let lines = puzzle_input
        .split("\n")
        .map(|line| parse_line(line))
        .collect::<Vec<(String, Vec<usize>)>>();
    println!("lines: {}", lines.len());
    print!("{:?}", lines.get(0).unwrap())
}

