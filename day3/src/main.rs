use std::{collections::HashMap, collections::HashSet, fs, vec};

fn boarders(idx: usize, row_num: usize, line: &Vec<char>) -> HashSet<(usize, usize)> {
    let mut vec: HashSet<(usize, usize)> = HashSet::new();
    if line[idx - 1] == '*' {
        vec.insert((row_num -1 , idx - 2));
    }
    if line[idx] == '*' {
        vec.insert((row_num -1 , idx -1));
    }
    if line[idx + 1] == '*' {
        vec.insert((row_num -1 , idx));
    }
    vec
}
fn find_num(
    line_number: usize,
    idx: usize,
    prev_line: &Vec<char>,
    line: Vec<char>,
    next_line: &Vec<char>,
) -> (u32, usize, HashSet<(usize, usize)>) {
    let mut index = idx;
    let mut num: u32 = 0;
    let mut vec: HashSet<(usize, usize)> = HashSet::new(); 
    while index < line.len() && line[index].is_numeric() {
        vec = vec.union(&boarders(index, line_number - 1, &prev_line)).cloned().collect();
        vec = vec.union(&boarders(index, line_number, &line)).cloned().collect();
        vec = vec.union(&boarders(index, line_number + 1, &next_line)).cloned().collect();
        num *= 10;
        num += (line[index].to_string()).parse::<u32>().unwrap();
        index += 1;
       
    }
        
    return (num, index - 1, vec);
}
fn sum_per_line(
    line_number: usize,
    prev_line: Vec<char>,
    line: Vec<char>,
    next_line: Vec<char>,
    star_map: &mut HashMap<(usize, usize), Vec<u32>>,
){
    let mut i: usize = 0;

    for c in line.clone() {
        if i == 0 || i == line.len() - 1 {
            i += 1;
            continue;
        }
        if c.is_numeric() {
            let (num, idx, vec) = find_num(
                line_number,
                i,
                &prev_line,
                line.clone(),
                &next_line,
            );
            for entry in vec{
                let mut temp_vec = star_map.get(&entry).unwrap_or(&Vec::new()).clone();
                temp_vec.push(num); 
                star_map.insert(entry, temp_vec);
            }
            i = idx;
        }
        i += 1;
    }

}
fn main() {
    let file_path: &str = "input.txt";
    let mut star_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum: u32 = 0;
    let mut char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(0, '.');
            chars.push('.');
            chars
        })
        .collect();
    let n: usize = char_matrix.len() + 2;
    let my_vector: Vec<char> = vec!['.'; char_matrix[0].len()];
    char_matrix.insert(0, my_vector.clone());
    char_matrix.push(my_vector.clone());
    for idx in 1..n - 1 {
        sum_per_line(
            idx,
            char_matrix[idx - 1].clone(),
            char_matrix[idx].clone(),
            char_matrix[idx + 1].clone(),
            &mut star_map,
        );         
    }
    let test: Vec<u32> = star_map.values().map(|vec|{ if vec.len() == 2{ vec[0]* vec[1]} else {0}}).collect();
    sum = test.iter().sum();
    println!("{:?}", sum);
}
