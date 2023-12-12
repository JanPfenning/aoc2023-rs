use std::fs::{self};

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d12/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn run_length_encoding(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut zeros = 0;
    let mut ones = if n & 1 == 1 {1} else {0};
    let mut rle = Vec::new();

    if n == 0 { return vec![1] }
    if n & 1 != 0 {
        rle.push(0);
    } else {
        n = n * 2;
    }

    n >>= 1;
    while n > 0 {
        if n & 1 == 1 {
            if zeros != 0 {
                rle.push(zeros);
                zeros = 0;
            }
            ones += 1;
        } else {
            if ones != 0 {
                rle.push(ones);
                ones = 0;
            }
            zeros += 1;
        }
        n >>= 1;
    }
    if ones != 0 {
        rle.push(ones);
    }
    rle.reverse();
    rle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_length_encoding() {
        assert_eq!(run_length_encoding(0b100111011), vec![1,2,3,1,2,0]); 
        assert_eq!(run_length_encoding(0b10101), vec![1,1,1,1,1,0]); 
        assert_eq!(run_length_encoding(0b10100), vec![1,1,1,2]); 
        assert_eq!(run_length_encoding(0b1111), vec![4,0]); 
        assert_eq!(run_length_encoding(0b10000), vec![1,4]); 
        assert_eq!(run_length_encoding(0b0), vec![1]); 
    }
}

pub fn p1() {
    let puzzle_input = read_puzzle_input();
    let lines = puzzle_input.split("\n").collect::<String>();
}

pub fn p2() {
    let puzzle_input = read_puzzle_input();
}