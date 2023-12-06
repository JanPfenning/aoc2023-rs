use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d05/puzzleinput_example.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    println!("{:?}", input_lines);
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct Chunk {
    name: String,
    translations: Vec<Translation>
}

fn parse_chunk_from_string(s: &str) -> Chunk {
    let all_lines = s.split("\n").into_iter().collect::<Vec<&str>>();
    let name = all_lines.get(0).unwrap().replace(" map:", "");
    let translation_lines = all_lines.get(1..=(all_lines.len()-1)).unwrap();
    let translations = translation_lines.iter().map(|line| {
        let parts = line.split(" ").into_iter().map(|val| val.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let [dest_start, source_start, len] = [parts.get(0).unwrap(), parts.get(1).unwrap(), parts.get(2).unwrap()];
        Translation {
            from: *source_start,
            to: source_start + len,
            summand: (*dest_start as isize) - (*source_start as isize),
        }
    }).collect();
    let chunk = Chunk {
        name, translations
    };
    println!("{chunk:?}");
    chunk
}

fn propagate_translation(base_translation: &Translation, follow_translations: &[Translation]) -> Vec<Translation> {
    let base_summand = base_translation.summand;
    let mut results = Vec::with_capacity(follow_translations.len());
    let mut last_to: usize = base_translation.from;

    let mut follow_translations_sorted = follow_translations.to_vec();
    follow_translations_sorted.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)));

    for trans in follow_translations_sorted {
        let is_outside_range = trans.from >= base_translation.to || trans.to <= base_translation.from;

        if !is_outside_range {
            let from = base_translation.from.max(trans.from);
            let to = base_translation.to.min(trans.to);

            if last_to < from {
                // Add filler translation with base summand if there is a gap
                results.push(Translation { from: last_to, to: from, summand: base_summand });
            }

            // Add the current translations's summand to base summand
            results.push(Translation { from, to, summand: trans.summand + base_summand });
            last_to = to;
        }
    }

    if last_to < base_translation.to {
        // Fill the gap between last_to and base_translation.to with base summand
        results.push(Translation { from: last_to, to: base_translation.to, summand: base_summand });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        base_range: Translation,
        translations: Vec<Translation>,
        expected: Vec<Translation>,
    }

    #[test]
    fn test_propagate_translation() {
        let fertilizer_to_water = vec![
            Translation { from: 0, to: 7, summand: 42 },
            Translation { from: 11, to: 53, summand: -11 },
            Translation { from: 7, to: 11, summand: 50 },
            Translation { from: 53, to: 61, summand: -4 },
        ];
        let test_cases = vec![
            TestCase {
                base_range: Translation {from: 5, to: 30, summand: 0},
                translations: vec![
                    Translation {from: 3, to: 18, summand: 2},
                    Translation {from: 20, to: 25, summand: -5},
                    Translation {from: 28, to: 35, summand: 3},
                ],
                expected: vec![
                    Translation {from: 5, to: 18, summand: 2},
                    Translation {from: 18, to: 20, summand: 0},
                    Translation {from: 20, to: 25, summand: -5},
                    Translation {from: 25, to: 28, summand: 0},
                    Translation {from: 28, to: 30, summand: 3},
                ],
            },
            TestCase {
                base_range: Translation {from: 5, to: 30, summand: 2},
                translations: vec![
                    Translation {from: 3, to: 18, summand: 3},
                    Translation {from: 20, to: 25, summand: -5},
                    Translation {from: 28, to: 35, summand: 1},
                ],
                expected: vec![
                    Translation {from: 5, to: 18, summand: 5},
                    Translation {from: 18, to: 20, summand: 2},
                    Translation {from: 20, to: 25, summand: -3},
                    Translation {from: 25, to: 28, summand: 2},
                    Translation {from: 28, to: 30, summand: 3},
                ],
            },
            TestCase {
                base_range: Translation { from: 50, to: 52, summand: -13 },
                translations: fertilizer_to_water.clone(),
                expected: vec![
                    Translation {from: 50, to: 52, summand: -24},
                ],
            },
            TestCase {
                base_range: Translation { from: 52, to: 54, summand: -13 },
                translations: fertilizer_to_water.clone(),
                expected: vec![
                    Translation {from: 52, to: 53, summand: -24},
                    Translation {from: 53, to: 54, summand: -17},
                ],
            },
            TestCase {
                base_range: Translation { from: 54, to: 98, summand: 2 },
                translations: fertilizer_to_water.clone(),
                expected: vec![
                    Translation {from: 54, to: 61, summand: -2},
                    Translation {from: 61, to: 98, summand: 2},
                ],
            },
            TestCase {
                base_range: Translation { from: 98, to: 100, summand: -48 },
                translations: fertilizer_to_water.clone(),
                expected: vec![
                    Translation {from: 98, to: 100, summand: -48},
                ],
            },
        ];

        for case in test_cases {
            let result = propagate_translation(&case.base_range, &case.translations);
            assert_eq!(result, case.expected);
        }
    }
}

fn merge_chunks(chunk_from: &Chunk, chunk_to: &Chunk) -> Chunk {
    let chunk_from_name_parts = chunk_from.name.split("-").into_iter().collect::<Vec<&str>>();
    let chunk_to_name_parts = chunk_to.name.split("-").into_iter().collect::<Vec<&str>>();
    if chunk_from_name_parts.get(2).unwrap() != chunk_to_name_parts.get(0).unwrap() { panic!("incompatible chunks")}
    let from = chunk_from_name_parts.get(0).unwrap().to_string();
    let to = chunk_to_name_parts.get(2).unwrap().to_string();
    let name = format!("{from}-to-{to}");
    
    let propagated_translations = chunk_from.translations.iter()
        .map(|translation| {
            propagate_translation(translation, &chunk_to.translations)
        })
        .collect::<Vec<Vec<Translation>>>();
    Chunk {
        name, 
        translations: propagated_translations.into_iter().flatten().collect()
    }
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    let seeds: Vec<usize> = input_lines.get(0).unwrap()
        .split(": ").into_iter().collect::<Vec<_>>().get(1).unwrap()
        .split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let chunks: Vec<String> = input_lines.join("\n").split("\n\n").skip(1).map(|s| s.to_string()).into_iter().collect();
    let chunks: Vec<Chunk> = chunks.iter().map(|chunk| parse_chunk_from_string(chunk)).collect();
    //chunks.iter().for_each(|chunk| println!("{:?}", chunk));
    let seed_to_fertilizer = merge_chunks(chunks.get(0).unwrap(), chunks.get(1).unwrap());
    //println!("{:?}", seed_to_fertilizer);
    let seed_to_water = merge_chunks(&seed_to_fertilizer, chunks.get(2).unwrap());
    //println!("{:?}", seed_to_water);
    let seed_to_light = merge_chunks(&seed_to_water, chunks.get(3).unwrap());
    //println!("{:?}", seed_to_light);
    let seed_to_temperature = merge_chunks(&seed_to_light, chunks.get(4).unwrap());
    //println!("{:?}", seed_to_temperature);
    let seed_to_humidity = merge_chunks(&seed_to_temperature, chunks.get(5).unwrap());
    //println!("{:?}", seed_to_humidity);
    let seed_to_location = merge_chunks(&seed_to_humidity, chunks.get(6).unwrap());
    println!("{:?}", seed_to_location.name);
    let mut translations = seed_to_location.translations;
    translations.sort_by(|translation_a, translation_b| translation_a.from.cmp(&translation_b.from));
    
    // TODO translation does not translate 82 to 46 but 82 to 49
    translations.iter().for_each(|translation| println!("{translation:?}"));
    
    // Adjust each seed range based on the translations
    let seed_ranges: Vec<(usize, usize)> = seeds.chunks_exact(2).map(|pair| (pair[0], pair[0]+pair[1])).collect();
    println!("number of seeds: {}", seed_ranges.iter().map(|(from, to)| to-from).fold(0, |acc, iter| acc + iter));
    // let seed_ranges = seed_ranges.iter().map(|(from, to)| Translation {
    //     from: *from,
    //     to: *to,
    //     summand: 0,
    // }).collect::<Vec<Translation>>();
    println!("seed ranges: {seed_ranges:?}");
    //let result = apply_translation_to_get_smallest_location_for_seed_in_range(seed_ranges, translations.into());
    let result = find_smallest_location(seed_ranges, translations.into());
    println!("result {result:?}");
    // i have a list of translations that all map seed ids to location ids.
    // Each entry is encoded as an object with (from: the first seed id that is eligable for this translation) (to: the last seed id that can be translated with this - exclusive) and (summand - the value added to any of the seeds in range to retreive the corresponding location)

    // given a list of seed ranges encoded as each (from, to) 

    // how to find the smallest location that is possible to retrieve with one of the seeds described by the seedrange using any of the translations?
}

fn find_smallest_location(seed_ranges: Vec<(usize, usize)>, translations: Vec<Translation>) -> Option<isize> {
    let mut smallest_location: Option<isize> = None;

    let mut seed_index = 0;
    let mut trans_index = 0;
    
    while seed_index < seed_ranges.len() && trans_index < translations.len() {
        let (from_seed, to_seed) = seed_ranges[seed_index];
        let trans = translations[trans_index];

        // If the seed range and translation overlap 
        if !(to_seed <= trans.from || from_seed >= trans.to) {
            // Get the overlapping part of the seed range and the translation
            let overlap_from = std::cmp::max(from_seed, trans.from);
            let overlap_to = std::cmp::min(to_seed, trans.to);

            // Calculate the location from the overlapping range
            for seed in overlap_from..overlap_to {
                let location = seed as isize + trans.summand;
                smallest_location = match smallest_location {
                    None => Some(location),
                    Some(val) => Some(val.min(location)),
                };
            }
            
            // If the seed range ends before or at the same point as the translation, go to next seed range
            if to_seed <= trans.to {
                seed_index += 1;
            }

            // If the translation ends before or at the same point as the seed range, go to next translation
            if trans.to <= to_seed {
                trans_index += 1;
            }
        } else if to_seed <= trans.from {
            // If the seed range is completely before the translation, go to next seed range
            seed_index += 1;
        } else if from_seed >= trans.to {
            // If the translation is completely before the seed range, go to next translation
            trans_index += 1;
        }
    }

    smallest_location
}
