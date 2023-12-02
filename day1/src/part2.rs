use std::{fs, vec};
use std::collections::HashMap;
fn find_ref_value(input: &mut String) -> i32{
    println!("{}", input);
    let written = find_first_writen_num(input);
    let digit = find_first_num(input, false);
    let first = if written[1] < digit[1] {
        written[0]
    }
    else{
        digit[0]
    };
    let written_last = find_last_writen_num(input);
    let digit_last = find_first_num(input, true);
    let last = if written_last[1] > digit_last[1] {
        written_last[0]
    }
    else{
        digit_last[0]
    };

    let res = first* 10 + last;
    println!("{}", res);
    return  res;
}
fn find_first_num(input: &mut String, reverse: bool) -> Vec<i32>{
    let iterate: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(input.chars().rev())
    } else {
        Box::new(input.chars())
    };
    let mut i: i32 = if reverse {(input.len() -1).try_into().unwrap()} else {0};
    for c in iterate {
        if c.is_numeric(){
            let num:i32 = (c.to_string()).parse::<i32>().unwrap();
            println!("{}, {}", num, i);
            let v :Vec::<i32> = vec![num, i];
            return v;
        }
        
        i = if reverse{i -1} else {i+1};
    }
    let val: i32 = if reverse {0} else {10};
    return vec![0, val];
}

fn find_first_writen_num(input: &mut str) -> Vec<i32>{
    let written_num_to_int: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut idx: i32 = i32::MAX;
    let mut min_key: &str = "";
    for (key, &values) in written_num_to_int.iter(){
        let temp_idx: i32 = if input.find(key).is_some(){
            input.find(key).unwrap().try_into().unwrap()
        } else {
            i32::MAX
        };
        if idx > temp_idx{
            idx = temp_idx;
            min_key = key;
        }
    }
    println!("{}, {}", min_key, idx);
    let val: i32 = *written_num_to_int.get(min_key).unwrap_or(&i32::MAX);
    let v: Vec<i32> = vec![val , idx];
    v
}

fn find_last_writen_num(input: &mut str) -> Vec<i32>{
    let written_num_to_int: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut idx: i32 = 0;
    let mut min_key: &str = "";
    for (key, &values) in written_num_to_int.iter(){
        let temp_idx: i32 = if input.rfind(key).is_some(){
            input.rfind(key).unwrap().try_into().unwrap()
        } else {
            0
        };
        if idx < temp_idx{
            idx = temp_idx;
            min_key = key;
        }
    }
    println!("{}, {}", min_key, idx);
    let val: i32 = *written_num_to_int.get(min_key).unwrap_or(&0);
    let v: Vec<i32> = vec![val , idx];
    v
}

fn main() {
    let file_path : &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: i32 = 0;
    for line in contents.lines(){
        let r1 = &mut String::from(line);
        sum += find_ref_value(r1);

    } 
    println!("{}", sum);       
}
