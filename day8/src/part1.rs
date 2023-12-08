use std::{fs, collections::HashMap};
use phf::phf_map;
const START: &str = "AAA";
const END: &str = "ZZZ";


static LR: phf::Map<char, usize> = phf_map! {
    'L' => 0,
    'R' => 1,
};
fn create_tree_map(contents: String) -> HashMap<String, Vec<String>>{
    let mut map: HashMap<String, Vec<String>> = HashMap::new(); 
    for line in contents.lines(){
        if line.contains("="){
            let new_line: String = line.replace(" ", "").replace("(", "").replace(")", "");
            println!("{}", new_line);
            let key: String = new_line.split("=").nth(0).unwrap().to_string();
            let val1: String = new_line.split("=").nth(1).unwrap().split(",").nth(0).unwrap().to_string();
            let val2: String = new_line.split("=").nth(1).unwrap().split(",").nth(1).unwrap().to_string();
            map.insert(key, vec![val1, val2]);
        }
    }
    map
     
}

fn find_target(tree_map: &HashMap<String, Vec<String>>, instractions: &Vec<char>) -> u32{
    let mut count: u32 = 0;
    let mut target: &str = START;
    while  target != END{
        for instraction in instractions{
            count += 1;
            target = &tree_map[target][*LR.get(instraction).unwrap()];
            if target == END{
                return count;
            }
        }
    }
    count  
}
fn main() {
    let file_path : &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: u32 = 0;
    let instractions:Vec<char> = contents.lines().nth(0).unwrap().chars().collect();
    let tree_map = create_tree_map(contents);
    println!("{}", find_target(&tree_map, &instractions));
 
}
