use std::fs;
fn to_int(num: String) -> u64 {
    num.parse().unwrap()
}

fn format_line(line: &str) -> u64 {
    let num: String = line
        .split(":")
        .nth(1)
        .unwrap().replace(" ", "");
    to_int(num)
}

fn find_all_win_in_game(time: u64, dist: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..(time / 2 + 1) {
        if i * (time - i) > dist {
            sum += 2;
        }
    }
    let res = if time % 2 == 1 { sum } else { sum - 1 };
    return res;
}

fn main() {
    let file_path: &str = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    for i in (0..lines.len() - 1).step_by(2) {
        let time: u64 = format_line(lines[i]);
        let dist: u64 = format_line(lines[i + 1]);
        println!("{}", find_all_win_in_game(time, dist));
    }
}
