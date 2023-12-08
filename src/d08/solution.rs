use std::fs;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d08/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn get_string_value(s: &str) -> usize {
    s.chars().rev().enumerate().map(|(i, c)| (c as usize - 'A' as usize) * (26usize.pow(i as u32))).sum()
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    //println!("{input_lines:#?}");
    // get first line, map each char in line to 0 if L or 1 if R 0> result vec<u8>
    let instructions = input_lines.get(0).unwrap().chars().map(|c| if c == 'L' {0} else {1}).collect::<Vec<u8>>();
    
    let mut arr: Vec<(usize, usize)> = vec![(0, 0); 26*26*26];
    for line in &input_lines[2..] {
        let sides: Vec<&str> = line.split(" = ").collect();
        let pos = get_string_value(sides[0]);
        let vals: Vec<usize> = sides[1].split(", ").map(|s| get_string_value(&s.replace("(","").replace(")",""))).collect();
        //println!("{vals:?}");
        arr[pos] = (vals[0], vals[1]);
    }
    arr.iter().enumerate().for_each(|(idx, line)| if line.0 > 0 || line.1 > 0 {
        println!("{idx}: {line:?}")
    });
    
    let mut i = 0;
    let mut cur_idx = 0;
    loop {
        let pos = instructions.get(i % instructions.len()).unwrap();
        println!("instruction: {}", pos);
        if *pos == 0 {
            cur_idx = arr[cur_idx].0;
        } else {
            cur_idx = arr[cur_idx].1;
        }
        if cur_idx == 17575 {
            break
        }
        i+=1;
    }
    i+=1;
    println!("iterations needed: {i}")
}

// 11A => 11B => 11Z => 11B => 11Z => 11B => 11Z
// 22A => 22B => 22C => 22Z => 22B => 22C => 22Z

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: isize, b: isize) -> isize {
    (a * b).abs() / gcd(a, b)
}

fn lcm_list(nums: &Vec<isize>) -> isize {
    nums.iter().fold(1, |a, &b| lcm(a, b))
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    //println!("{input_lines:#?}");
    // get first line, map each char in line to 0 if L or 1 if R 0> result vec<u8>
    let instructions = input_lines.get(0).unwrap().chars().map(|c| if c == 'L' {0} else {1}).collect::<Vec<u8>>();
    
    //let mut arr: Vec<(usize, usize)> = vec![(0, 0); 26*26*26];
    let mut arr: [(usize, usize); 26*26*26] = vec![(0, 0); 26*26*26].try_into().unwrap();
    for line in &input_lines[2..] {
        let sides: Vec<&str> = line.split(" = ").collect();
        let pos = get_string_value(sides[0]);
        let vals: Vec<usize> = sides[1].split(", ").map(|s| get_string_value(&s.replace("(","").replace(")",""))).collect();
        //println!("{vals:?}");
        arr[pos] = (vals[0], vals[1]);
    }
    arr.iter().enumerate().for_each(|(idx, line)| if line.0 > 0 || line.1 > 0 {
        println!("{idx}: {line:?}")
    });

    let start_a_list: Vec<_> = arr.iter().enumerate()
        .filter_map(|(i, line)| if i % 26 == 0 && (line.0 > 0 || line.1 > 0) { Some(i) } else { None })
        .collect();
    println!("{} starts", start_a_list.len());
    
    //let mut list_to_visited_indices_per_start = start_a_list.clone().iter().map(|start| vec![*start]).collect::<Vec<Vec<usize>>>();
    let mut last_visited_index_per_start = start_a_list.clone();
    let mut i = 0;

    let mut first_finished_at: Vec<usize> = vec![0; start_a_list.len()];
    while !first_finished_at.iter().all(|x| *x > 0) {
        last_visited_index_per_start.iter().enumerate().for_each(|(idx, last_visit)| if last_visit % 26 == 25 && last_visited_index_per_start[idx] != 0 {first_finished_at[idx] = i});
        let instruction = instructions.get(i % instructions.len()).unwrap();
        last_visited_index_per_start = last_visited_index_per_start.iter().map(|last_visit| {
            let next_val: usize = {
                if *instruction == 0 {
                    arr[*last_visit].0
                } else {
                    arr[*last_visit].1
                }
            };
            next_val
        }).collect();
        i+=1;
    }

    let result = lcm_list(&first_finished_at.iter().map(|x| *x as isize).collect());
    
    println!("{:?}", first_finished_at);
    println!("lcm {:?}", result);
}
