use std::{collections::HashMap, fs};
fn get_seeds(file_path: &str) -> Vec<u64> {
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut seeds: Vec<u64> = Vec::new();
    for line in contents.lines() {
        let new_line: &str = line.split(":").nth(1).unwrap();
        let mut temp: Vec<&str> = new_line.split(" ").collect();
        temp = temp[1..].to_vec();
        seeds = temp
            .into_iter()
            .map(|item| item.parse::<u64>().unwrap() as u64)
            .collect()
    }
    seeds
}

fn create_map(file_path: &str) -> (HashMap<u64, u64>, HashMap<u64, u64>) {
    let mut fert_map: HashMap<u64, u64> = HashMap::new();
    let mut range_map: HashMap<u64, u64> = HashMap::new();
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        let temp: Vec<&str> = line.split(" ").collect();
        let dest: u64 = temp[0].parse().unwrap();
        let source: u64 = temp[1].parse().unwrap();
        let range: u64 = temp[2].parse().unwrap();
        fert_map.insert(source, dest);
        range_map.insert(source, range);
    }
    (fert_map, range_map)
}

fn create_source_lst(
    fert_map: HashMap<u64, u64>,
    range_map: HashMap<u64, u64>,
    prev_source: Vec<u64>,
) -> Vec<u64> {
    let mut source_lst: Vec<u64> = Vec::new();
    for src in prev_source {
        let mut flag: bool = false;
        for (&key, &value) in range_map.iter() {
            if (src >= key) && (src < (key + value)) {
                source_lst.push(*fert_map.get(&key).unwrap() + (src - key));
                flag = true;
                break;
            }
        }
        if !flag {
            source_lst.push(src);
        }
    }
    return source_lst;
}

fn main() {
    let mut source: Vec<u64> = get_seeds("seeds.txt");
    let file_names: Vec<&str> = vec![
        "seet_to_soil.txt",
        "soil-to-fertilizer.txt",
        "fertilizer-to-water.txt",
        "water-to-light map.txt",
        "light-to-temperature.txt",
        "temperature-to-humidity.txt",
        "humidity-to-location.txt",
    ];
    for file_name in file_names{
        let (fert_map, range_map) = create_map(file_name);
        source = create_source_lst(fert_map, range_map, source);
    }
    let min_loc: u64 = source.into_iter().min().unwrap();
    println!("{}", min_loc)


}
