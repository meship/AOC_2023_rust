use std::{fs, collections::HashMap};
use phf::phf_map;
use num_integer::lcm;

static LR: phf::Map<char, usize> = phf_map! {
    'L' => 0,
    'R' => 1,
};
fn create_tree_map(contents: String) -> (HashMap<String, Vec<String>>, Vec<String>){
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut starts: Vec<String> = Vec::new(); 
    for line in contents.lines(){
        if line.contains("="){
            let new_line: String = line.replace(" ", "").replace("(", "").replace(")", "");
            println!("{}", new_line);
            let key: String = new_line.split("=").nth(0).unwrap().to_string();
            if key.ends_with('A'){
                starts.push(key.clone());
            }
            let val1: String = new_line.split("=").nth(1).unwrap().split(",").nth(0).unwrap().to_string();
            let val2: String = new_line.split("=").nth(1).unwrap().split(",").nth(1).unwrap().to_string();
            map.insert(key, vec![val1, val2]);
        }
          
    }

    (map, starts)
     

}

fn find_lcm(paths: Vec<u64>) -> u64{
    let mut ans: u64 = paths[0];
    for i in 1..paths.len(){
        ans = lcm(ans, paths[i]);
    }
    ans
}

fn find_target(tree_map: &HashMap<String, Vec<String>>, instractions: &Vec<char>, start: &String) -> u64{
    let mut count: u64 = 0;
    let mut target: &str = start;
    while  !target.ends_with('Z'){
        for instraction in instractions{
            count += 1;
            target = &tree_map[target][*LR.get(instraction).unwrap()];
            if target.ends_with('Z'){
                return count;
            }
        }
    }
    count  
}

fn find_steps_for_each_start(tree_map: &HashMap<String, Vec<String>>, instractions: &Vec<char>, starts: &Vec<String>) -> Vec<u64>{
    let mut steps_for_path: Vec<u64> = Vec::new(); 
    for  start in starts{
        steps_for_path.push(find_target(tree_map, instractions, start));
    }

    steps_for_path

}
// fn find_target(tree_map: &HashMap<String, Vec<String>>, instractions: &Vec<char>, starts: &mut Vec<String>) -> u64{
//     let mut count: u64 = 0;
//     for insraction in instractions{
//         count += 1;
//         for (idx, start) in starts.into_iter().enumerate(){
//             starts[idx] = tree_map[start][LR[insraction]];
//         }

//     }
//     count  
// }
fn main() {
    let file_path : &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let instractions:Vec<char> = contents.lines().nth(0).unwrap().chars().collect();
    let  (tree_map, starts) = create_tree_map(contents);
    let paths: Vec<u64> = find_steps_for_each_start(&tree_map, &instractions, &starts);
    println!("{}", find_lcm(paths));
 
}
