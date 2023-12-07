use std::{
    collections::{btree_map::Values, HashMap},
    fs,
};
fn to_int(num: &str) -> u32 {
    num.parse().unwrap()
}

fn format_line(line: &str) -> Vec<u32> {
    let vec: Vec<&str> = line
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|val| if !val.is_empty() { true } else { false })
        .collect();
    vec.into_iter().map(|num| to_int(num)).collect()
}

fn find_all_win_in_game(time: u32, dist: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..(time / 2 + 1) {
        if i * (time - i) > dist {
            sum += 2;
        }
    }
    let res = if time % 2 == 1 { sum } else { sum - 1 };
    println!("the sum is: {}", res);
    return res;
}

fn mult_all_wins(times: Vec<u32>, dists: Vec<u32>) -> u32 {
    let mut mult: u32 = 1;
    for (&time, &dist) in times.iter().zip(dists.iter()) {
        mult = mult * find_all_win_in_game(time, dist);
    }
    mult
}

fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    for i in (0..lines.len() - 1).step_by(2) {
        let times: Vec<u32> = format_line(lines[i]);
        let dists: Vec<u32> = format_line(lines[i + 1]);
        println!("{:?}, {:?}", times, dists);
        println!("{}", mult_all_wins(times, dists));
    }
}
