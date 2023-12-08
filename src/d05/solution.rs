use std::fs;
use std::path::PathBuf;

#[derive(Copy, Clone)]
struct Translation {
    from: usize,
    to: usize,
    summand: isize
}

impl PartialEq for Translation {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.summand == other.summand
    }
}

impl std::fmt::Debug for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{from: {}, to: {}, summand: {}}}", self.from, self.to, self.summand)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Chunk {
    name: String,
    translations: Vec<Translation>
}

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d05/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    let seeds: Vec<usize> = input_lines.get(0).unwrap()
        .split(": ").into_iter().collect::<Vec<_>>().get(1).unwrap()
        .split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let chunks: Vec<String> = input_lines.join("\n").split("\n\n").skip(1).map(|s| s.to_string()).into_iter().collect();
    let chunks: Vec<Chunk> = chunks.iter().map(|chunk| parse_chunk_from_string(chunk)).collect();
    println!("-----");

    let base_chunk = Chunk {
        name: "seed-to-seed".to_string(),
        translations: [Translation {
            from: 0,
            to: usize::MAX,
            summand: 0
        }].to_vec()
    };

    let seed_to_location = propagate_chunks_to_get_seed_to_location(base_chunk, chunks);
    
    let locations = seeds.into_iter()
        .map(|seed| get_destination_value_using_chunk(seed, &seed_to_location))
        .collect::<Vec<usize>>();
    let mut result = locations;
    result.sort();
    println!("{:?}", result);
    println!("result {:?}", result.get(0));
}

fn get_destination_value_using_chunk(val: usize, chunk: &Chunk) -> usize {
    let translation = (*chunk.translations).into_iter().find(|translation| translation.from<=val && translation.to > val);
    let x = val as isize + match translation {
        Some(trans) => { trans.summand },
        None =>{ 0 },
    };
    x as usize
}

fn propagate_single_translation(a: &Translation, b: &Translation) -> Option<Translation> {
    // shift a
    let a_lower_in_bs_view = {
        if a.summand > 0 {a.from.saturating_add(a.summand.abs() as usize)} else {a.from.saturating_sub(a.summand.abs() as usize)}
    };
    let a_upper_in_bs_view = {
        if a.summand > 0 {a.to.saturating_add(a.summand.abs() as usize)} else {a.to.saturating_sub(a.summand.abs() as usize)}
    };
    
    // get intersection range
    let intersection_lower = a_lower_in_bs_view.max(b.from);
    let intersection_upper = a_upper_in_bs_view.min(b.to);
    
    // shift back to view of a
    // shift a
    let from = {
        if a.summand > 0 {intersection_lower.saturating_sub(a.summand.abs() as usize)} else {intersection_lower.saturating_add(a.summand.abs() as usize)}
    };
    let to = {
        if a.summand > 0 {intersection_upper.saturating_sub(a.summand.abs() as usize)} else {intersection_upper.saturating_add(a.summand.abs() as usize)}
    };

    if to <= from { return None }
    
    // 
    let translation = Translation {
        from, to, summand: a.summand + b.summand
    };
    Some(translation)
}

fn propagate_chunks_to_get_seed_to_location(base_chunk: Chunk, chunks: Vec<Chunk>) -> Chunk {
    let seed_to_soil = merge_chunks(&base_chunk, chunks.get(0).unwrap());
    println!("{:#?}", seed_to_soil);
    let seed_to_fertilizer = merge_chunks(&seed_to_soil, chunks.get(1).unwrap());
    println!("{:#?}", seed_to_fertilizer);
    let seed_to_water = merge_chunks(&seed_to_fertilizer, chunks.get(2).unwrap());
    println!("{:#?}", seed_to_water);
    let seed_to_light = merge_chunks(&seed_to_water, chunks.get(3).unwrap());
    println!("{:#?}", seed_to_light);
    let seed_to_temperature = merge_chunks(&seed_to_light, chunks.get(4).unwrap());
    println!("{:#?}", seed_to_temperature);
    let seed_to_humidity = merge_chunks(&seed_to_temperature, chunks.get(5).unwrap());
    println!("{:#?}", seed_to_humidity);
    let seed_to_location = merge_chunks(&seed_to_humidity, chunks.get(6).unwrap());
    println!("{:#?}", seed_to_location);
    seed_to_location
}

fn parse_chunk_from_string(s: &str) -> Chunk {
    let all_lines = s.split("\n").into_iter().collect::<Vec<&str>>();
    let name = all_lines.get(0).unwrap().replace(" map:", "");
    let translation_lines = all_lines.get(1..=(all_lines.len()-1)).unwrap();
    let mut translations: Vec<Translation> = translation_lines.iter().map(|line| {
        let parts = line.split(" ").into_iter().map(|val| val.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let [dest_start, source_start, len] = [parts.get(0).unwrap(), parts.get(1).unwrap(), parts.get(2).unwrap()];
        Translation {
            from: *source_start,
            to: source_start + len,
            summand: (*dest_start as isize) - (*source_start as isize),
        }
    }).collect();
    translations.sort_by(|translation_a, translation_b| translation_a.from.cmp(&translation_b.from)); 
    
    // if first translations from is bigger than 0 => add a translation from 0 to the "from" with summand 0
    if translations[0].from > 0 {
        translations.insert(0, Translation {from: 0, to: translations[0].from, summand: 0 });
    }

    // any two translations are not back to back (exclusive to does not match the next inclusive "from") add a new translation filling the gap with summand 0
    let mut i = 1;
    while i < translations.len() {
        let prev = translations[i-1].clone();
        let cur = translations[i].clone();
        if cur.from != prev.to {
            let trans = Translation {
                from: prev.to,
                to: cur.from,
                summand: 0,
            };
            translations.insert(i, trans);
        }
        i += 1;
    }

    // if last translations "to" is less than usize::max add a translation from the last "to" to the max value with summand 0
    let last_trans = translations[translations.len() - 1].clone();
    if last_trans.to < std::usize::MAX {
        translations.push(Translation {from: last_trans.to, to: std::usize::MAX, summand: 0 });
    }
    
    let chunk = Chunk {
        name, translations
    };
    println!("{:#?}", chunk);
    chunk
}

fn merge_chunks(chunk_from: &Chunk, chunk_to: &Chunk) -> Chunk {
    let chunk_from_name_parts: Vec<&str> = chunk_from.name.split("-").collect();
    let chunk_to_name_parts: Vec<&str> = chunk_to.name.split("-").collect();
    if chunk_from_name_parts[2] != chunk_to_name_parts[0] { panic!("incompatible chunks")}
    let from = chunk_from_name_parts[0].to_string();
    let to = chunk_to_name_parts[2].to_string();
    let name = format!("{}-to-{}",from, to);
    
    let propagated_translations: Vec<Translation> = chunk_from.translations.iter()
        .flat_map(|translation| {
            chunk_to.translations.iter()
                .filter_map(|second| propagate_single_translation(translation, second))
        }).collect();

    let merged = Chunk {
        name, 
        translations: propagated_translations
    };
    //println!("merged {:#?}", merged);
    merged
}

#[cfg(test)]
mod merge_chunks_tests {
    use super::*;

    struct TestCase {
        from_chunk: Chunk,
        to_chunk: Chunk,
        expected_merged: Chunk,
    }

    #[test]
    fn test_propagate_translation() {
        
        let test_cases = vec![
            TestCase {
                from_chunk: Chunk {
                    name: "x-to-y".to_string(),
                    translations: vec![
                        Translation { from: 4, to: 13, summand: 2 },
                        Translation { from: 13, to: 16, summand: -2 }
                    ]
                },
                to_chunk: Chunk {
                    name: "y-to-z".to_string(),
                    translations: vec![
                        Translation { from: 4, to: 8, summand: -4 },
                        Translation { from: 8, to: 11, summand: 3 },
                        Translation { from: 11, to: 13, summand: 1 },
                        Translation { from: 13, to: 20, summand: -2 }
                    ]
                },
                expected_merged: Chunk {
                    name: "x-to-z".to_string(),
                    translations: vec![
                        Translation { from: 4, to: 6, summand: -2 },
                        Translation { from: 6, to: 9, summand: 5 },
                        Translation { from: 9, to: 11, summand: 3 },
                        Translation { from: 11, to: 13, summand: 0 },
                        Translation { from: 13, to: 15, summand: -1 },
                        Translation { from: 15, to: 16, summand: -4 },
                    ]
                }
            }
        ];

        for case in test_cases {
            let result = merge_chunks(&case.from_chunk, &case.to_chunk);
            assert_eq!(result, case.expected_merged);
        }
    }
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    let seeds: Vec<usize> = input_lines.get(0).unwrap()
        .split(": ").into_iter().collect::<Vec<_>>().get(1).unwrap()
        .split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let chunks: Vec<String> = input_lines.join("\n").split("\n\n").skip(1).map(|s| s.to_string()).into_iter().collect();
    let chunks: Vec<Chunk> = chunks.iter().map(|chunk| parse_chunk_from_string(chunk)).collect();
    
    let seed_ranges: Vec<(usize, usize)> = seeds.chunks_exact(2).map(|pair| (pair[0], pair[0]+pair[1])).collect();

    let base_chunk = Chunk {
        name: "seed-to-seed".to_string(),
        translations: seed_ranges.iter().map(|(from, to)| Translation { from: *from, to: *to, summand: 0}).collect()
    };

    let seed_to_location = propagate_chunks_to_get_seed_to_location(base_chunk, chunks);

    let range_results = seed_to_location.translations.iter().map(|translation| {
        let location_start_of_range = {
            if translation.summand > 0 {translation.from.saturating_add(translation.summand.abs() as usize)} else {translation.from.saturating_sub(translation.summand.abs() as usize)}
        };
        let seed_sart_of_range = translation.from;
        (seed_sart_of_range, location_start_of_range)
    });
    let seed_start_range_with_minimal_loation_start = range_results.min_by(|a, b| a.1.cmp(&b.1));
    println!("{seed_start_range_with_minimal_loation_start:?}");
}