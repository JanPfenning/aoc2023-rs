use std::fs;
use std::path::PathBuf;
use std::cmp::PartialEq;
use std::collections::HashMap;

fn read_puzzle_input() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = root.join("src/d04/puzzleinput.txt");
    //println!("{}", file_path.to_string_lossy());
    let contents = fs::read_to_string(file_path)
        .expect(format!("Something went wrong reading the file").as_str());
    contents
}

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: [u8; 10],
    played_numbers: [u8; 25],
}

fn parse_line(line: &str) -> ScratchCard {
    let parts = line.split(": ").collect::<Vec<&str>>().get(1).unwrap().split(" | ").collect::<Vec<&str>>();
    let winning_numbers = parts.get(0).unwrap().split_whitespace().map(|number_string| number_string.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let played_numbers = parts.get(1).unwrap().split_whitespace().map(|number_string| number_string.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    ScratchCard {
        winning_numbers: TryInto::<[u8; 10]>::try_into(winning_numbers).unwrap(),
        played_numbers: TryInto::<[u8; 25]>::try_into(played_numbers).unwrap(),
    }
}

fn get_common_elements<T: Clone + PartialEq>(x: &Vec<T>, y: &Vec<T>) -> Vec<T>{
    x.iter().filter(|val| y.contains(val)).map(|val| val.to_owned()).collect::<Vec<T>>()
}

fn get_worth_of_played_winning_numbers(played_winning_numbers: &Vec<u8>) -> u16 {
    if played_winning_numbers.len() > 0 {
        2_u16.pow((played_winning_numbers.len()-1).try_into().unwrap())
    }
     else {
        0
    }
}

pub fn p1() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    let cards = input_lines.iter().map(|line| parse_line(line)).collect::<Vec<ScratchCard>>();
    //println!("{:?}", cards);
    let played_winning_numbers_per_card = cards.iter().map(|card| get_common_elements::<u8>(&card.winning_numbers.to_vec(), &card.played_numbers.to_vec())).collect::<Vec<Vec<u8>>>();
    let worths = played_winning_numbers_per_card.iter().map(|played_winning_numbers| get_worth_of_played_winning_numbers(played_winning_numbers)).collect::<Vec<u16>>();
    println!("{:?}", worths);
    println!("{:?}", worths.iter().fold(0, |acc, iter| acc + iter));
}

pub fn p2() {
    let input_lines = read_puzzle_input().split("\n").map(|val| val.to_owned()).collect::<Vec<String>>();
    let cards = input_lines.iter().map(|line| parse_line(line)).collect::<Vec<ScratchCard>>();
    
    let mut map: HashMap<u8, usize> = HashMap::new();
    let card_original_values = cards.iter().enumerate().map(|(idx, _card)| get_number_of_cards(&cards, 1_u8 + TryInto::<u8>::try_into(idx).unwrap(), &mut map));
    //println!("{:?}", card_original_values);
    println!("{:?}", card_original_values.fold(0, |acc, iter| acc + iter));
}

fn get_played_winning_numbers(card: &ScratchCard) -> Vec<u8> {
    get_common_elements::<u8>(&card.winning_numbers.to_vec(), &card.played_numbers.to_vec())
}

fn get_number_of_cards(cards: &Vec<ScratchCard>, id_of_card_in_question: u8, mut cache: &mut HashMap<u8, usize>) -> usize {
    if match cache.get(&id_of_card_in_question) {
        Some(_) => {true},
        None => {false},
    } { 
        //println!("cache hit for {id_of_card_in_question}");
        return *(cache.get(&id_of_card_in_question).unwrap()); 
    } else {
        println!("cache miss for {id_of_card_in_question}\nstart calculation")
    }

    let id_of_card_in_question = usize::from(id_of_card_in_question);
    if id_of_card_in_question > cards.len() { 
        println!("found 0 as value for {id_of_card_in_question}");
        cache.insert(id_of_card_in_question.try_into().unwrap(), 0);
        return 0;
     }

    let cur_card = cards.get(id_of_card_in_question-1).expect("no card found for id");
    let copies_to_receive = get_played_winning_numbers(&cur_card).len();
    println!("receive {copies_to_receive} new cards");

    if copies_to_receive == 0 {
        println!("found 1 as value for {id_of_card_in_question}"); 
        cache.insert(id_of_card_in_question.try_into().unwrap(), 1);
        return 1;
     }

    let ids_to_check = (id_of_card_in_question+1)..=(id_of_card_in_question+copies_to_receive);
    println!("Ids to add for {id_of_card_in_question}: {:?}", ids_to_check);
    
    let value = 1 + ids_to_check.map(|id| {
        //println!("recursive search for {id}");
        get_number_of_cards(&cards, id.try_into().unwrap(), &mut cache)
    }).fold(0, |acc, iter| acc + iter);
    println!("found {value} as value for {id_of_card_in_question}");
    cache.insert(id_of_card_in_question.try_into().unwrap(), value);
    value
}
