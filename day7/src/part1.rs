use std::{fs, collections::HashMap};
use phf::phf_map;

static CARD_TO_RANK: phf::Map<char, u16> = phf_map! {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
};

fn char_to_int(digit: char) -> u16{
    digit.to_string().parse().unwrap_or(0)
}

fn card_to_num(card: char) -> u16{
    *CARD_TO_RANK.get(&card).unwrap_or(&char_to_int(card))
}

fn format_line(line: &str) -> (&str, u32) {
    let vec: Vec<&str> = line
        .split(" ")
        .filter(|val| if !val.is_empty() { true } else { false })
        .collect();
    (vec[0], vec[1].parse().unwrap())
    
}

fn create_count_map(input_str: &str) -> HashMap<char, u16> {
    let mut count_map = HashMap::new();
    for ch in input_str.chars() {
        let counter = count_map.entry(ch).or_insert(0);
        *counter += 1;
    }
    count_map
}

fn hand_to_calss(hand: &str) -> u16{
    let count_map: HashMap<char, u16> = create_count_map(hand);
    let mut vals: Vec<u16> = count_map.values().cloned().collect();
    vals.sort_by(|a, b| b.cmp(a));
    let val: u16 = match vals[0]{
        5 => 7,
        4 => 6,
        3 => if vals[1] == 2 {5} else {4},
        2 => if vals[1] == 2 {3} else {2},
        1 => 1, 
        _ => 0,

    };
    val
}

fn hands_cmp(hand1: &str, hand2: &str) -> std::cmp::Ordering{
    for (char1, char2) in hand1.chars().zip(hand2.chars()){
        if card_to_num(char1) > card_to_num(char2){
            return std::cmp::Ordering::Greater;
        } else if card_to_num(char1) < card_to_num(char2){
            return std::cmp::Ordering::Less;
        }
       
    }
    return std::cmp::Ordering::Equal; 
}


fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut had_to_bid: HashMap<&str, u32> = HashMap::new();
    let mut hands: Vec<(&str, u16)> = Vec::new();
    for line in contents.lines() {
        let (hand, bid) = format_line(line);
        had_to_bid.insert(hand, bid);
        hands.push((hand, hand_to_calss(hand)));
        
    }

    hands.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| hands_cmp(a.0, b.0)));
    let mut sum: u32 = 0;
    for i in 0..hands.len(){
        sum += ((i+1) as u32)*had_to_bid.get(hands[i].0).unwrap();
    }
    println!("{}", sum);
    
}
