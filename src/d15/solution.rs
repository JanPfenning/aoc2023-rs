use std::fs;

fn read_puzzle_input() -> String {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d15/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

fn lens_hash(instruction: &str) -> usize{
    instruction.chars().fold(0, |acc, iter| ((acc + iter as usize) * 17) % 256 )
}

pub fn p1() {
    let puzzle_input: String = read_puzzle_input();
    let instruction_hash_values = puzzle_input.split(",").map(|instruction| {
        lens_hash(instruction)
    }).collect::<Vec<usize>>();
    println!("{instruction_hash_values:?}");
    println!("result {}", instruction_hash_values.iter().fold(0, |acc, iter| acc + iter));
}

fn operation_minus(label: String, box_: &mut BoxSlots) -> &mut BoxSlots {
    *box_ = box_.iter()
        .filter_map(|box_lens| if *box_lens.0 != label { Some(box_lens.clone()) } else { None })
        .collect::<BoxSlots>();
    box_
}

fn operation_equals(lens: Lens, box_: &mut BoxSlots) -> &mut BoxSlots {
    let found_lens_position = box_.clone().iter().position(|box_lens| 
        lens.0 == box_lens.0
    );

    if let Some(index) = found_lens_position {
        // replace value at index with lens from parameter
        box_[index] = lens;
    } else {
        // insert lens from parameter at end
        box_.push(lens);
    }

    box_
}

type Lens = (String, usize);
type BoxSlots = Vec<Lens>;
pub fn p2() {
    assert_eq!(lens_hash("rn"), 0);
    let puzzle_input = read_puzzle_input();
    let mut boxes: Vec<BoxSlots> = vec![Vec::new(); 256];
    puzzle_input.split(",").for_each(|instruction| {
        //println!("\nworking on instruction {:?}", instruction);
        let instruction = instruction.to_string();
        if instruction.ends_with("-") {
            let label = &instruction[0..instruction.len()-1];
            let box_idx = lens_hash(&label);
            // println!("box {box_idx} before remove: {:?}", boxes[box_idx]);
            boxes[box_idx] = operation_minus(label.to_owned(), &mut boxes[box_idx]).to_vec();
            // println!("box {box_idx} after remove: {:?}", boxes[box_idx]);
        } else {
            let mut iter = instruction.split("=");
            let label = iter.next().unwrap();
            let focal = iter.next().unwrap().parse::<usize>().unwrap();

            let box_idx = lens_hash(&label);
            // println!("box {box_idx} before add/replace: {:?}", boxes[box_idx]);
            boxes[box_idx] = operation_equals((label.to_owned(), focal), &mut boxes[box_idx]).to_vec();
            // println!("box {box_idx} after add/replace: {:?}", boxes[box_idx]);
        }
    });

    // println!("\nfirt few boxes");
    // println!("{:?}", boxes[0]);
    // println!("{:?}", boxes[1]);
    // println!("{:?}", boxes[2]);
    // println!("{:?}", boxes[3]);

    let result = boxes.iter().enumerate()
        .map(|(box_idx, box_)| box_.iter().enumerate()
            .fold(0, |acc, (slot_idx, iter)| acc + (iter.1 * (box_idx+1) * (slot_idx+1)))
        ).fold(0, |acc, box_value| acc + box_value);
    println!("{:?}",result);
}