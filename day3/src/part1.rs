use std::fs;

fn find_num(
    idx: usize,
    prev_line: &Vec<char>,
    line: Vec<char>,
    next_line: &Vec<char>,
) -> (u32, usize) {
    let mut index = idx;
    let mut flag: bool = false;
    let specieal = |c: char| !(c.is_numeric() || c == '.');
    let boarders = |idx: usize, line: &Vec<char>| {
        specieal(line[idx - 1]) || specieal(line[idx + 1]) || specieal(line[idx])
    };
    let mut num: u32 = 0;
    while index < line.len() && line[index].is_numeric() {
        if boarders(index, &line) || boarders(index, prev_line) || boarders(index, next_line) {
            flag = true;
        }
        num *= 10;
        num += (line[index].to_string()).parse::<u32>().unwrap();
        index += 1;
    }
    if flag {
        println!("the cur num is: {}", num);
        return (num, index-1);
    }
    (0, index - 1)
}
fn sum_per_line(prev_line: Vec<char>, line: Vec<char>, next_line: Vec<char>) -> u32 {
    let mut sum_per_row: u32 = 0;
    let mut i: usize = 0;

    for c in line.clone() {
        if i == 0 || i == line.len() - 1 {
            i += 1;
            continue;
        }
        if c.is_numeric() {
            let (num, idx) = find_num(i, &prev_line, line.clone(), &next_line);
            sum_per_row += num;
            i = idx;
        }
        i += 1;
    }
    sum_per_row
}
fn main() {
    let file_path: &str = "input.txt";
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
    println!("{:?}", char_matrix);
    for idx in 1..n - 1 {
        sum += sum_per_line(
            char_matrix[idx - 1].clone(),
            char_matrix[idx].clone(),
            char_matrix[idx + 1].clone(),
        );
    }
    println!("{:?}", sum);
}
