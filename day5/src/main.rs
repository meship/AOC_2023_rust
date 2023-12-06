use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;
fn get_seeds(file_path: &str) -> HashMap<u64, u64> {
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
    let mut range_src_map: HashMap<u64, u64> = HashMap::new();
    for i in (0..seeds.len()).step_by(2) {
        range_src_map.insert(seeds[i], seeds[i] + seeds[i + 1] - 1);
    }
    range_src_map
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
        range_map.insert(source, source + range - 1);
    }
    (fert_map, range_map)
}

fn create_source_lst(
    fert_map: HashMap<u64, u64>,
    range_map: HashMap<u64, u64>,
    prev_source: HashMap<u64, u64>,
) -> HashMap<u64, u64> {
    println!("Started create_source_lst {:?}", prev_source);
    let mut source_map: HashMap<u64, u64> = HashMap::new();
    for (src_start, src_end) in prev_source {
        println!(
            " > Entered loop, current interval: {:?}",
            (src_start, src_end)
        );
        let mut no_mutch: VecDeque<(u64, u64)> = VecDeque::new();
        no_mutch.push_back((src_start, src_end));
        let mut length: i32 = -1;
        while !no_mutch.is_empty() && length != source_map.len() as i32 {
            length = source_map.len() as i32;
            let (cur_src_start, cur_src_end) = no_mutch[0];
            println!(" @@@ Current no match: {:?}", no_mutch);
            for (&map_src_start, &map_src_end) in range_map.iter() {
                let val: u64 = *fert_map.get(&map_src_start).unwrap();
                if map_src_start < cur_src_start && map_src_end > cur_src_end {
                    println!(" >> Case 1 ({}, {})", map_src_start, map_src_end);
                    let start: u64 = val + (cur_src_start - map_src_start);
                    let end: u64 = start + (cur_src_end - cur_src_start);
                    no_mutch.pop_front();
                    if start <= end {
                        println!(" >> Pushing to    match ({}, {})", start, end);
                        source_map.insert(start, end);
                    }
                } else if cur_src_start <= map_src_start && cur_src_end >= map_src_end {
                    println!(" >> Case 4 ({}, {})", map_src_start, map_src_end);
                    no_mutch.pop_front();
                    if map_src_start - 1 >= cur_src_start {
                        println!(
                            " >> Pushing to no-match ({}, {})",
                            cur_src_start,
                            map_src_start - 1
                        );
                        no_mutch.push_back((cur_src_start, map_src_start - 1));
                    }
                    if map_src_end + 1 <= cur_src_end {
                        no_mutch.push_back((map_src_end + 1, cur_src_end));
                        println!(
                            " >> Pushing to no-match ({}, {})",
                            map_src_end + 1,
                            cur_src_end
                        );
                    }
                    let end = val + (map_src_end - map_src_start);
                    println!(" >> Pushing to    match ({}, {})", val, end);
                    source_map.insert(val, end);
                } else if cur_src_start <= map_src_start && map_src_start <= cur_src_end {
                    println!(" >> Case 2 ({}, {})", map_src_start, map_src_end);
                    no_mutch.pop_front();
                    println!("popped: {:?}", no_mutch);
                    if map_src_start > 0 && map_src_start - 1 >= cur_src_start {
                        println!(
                            " >>> Pushing to no-match ({}, {})",
                            cur_src_start,
                            map_src_start - 1
                        );
                        no_mutch.push_back((cur_src_start, map_src_start - 1));
                        println!("pushed: {:?}", no_mutch);
                    }
                    println!(
                        " >>> Pushing to    match ({}, {})",
                        val,
                        val + (cur_src_end - map_src_start)
                    );
                    source_map.insert(val, val + (cur_src_end - map_src_start));
                } else if cur_src_start >= map_src_start
                    && map_src_end <= cur_src_end
                    && map_src_end >= cur_src_start
                {
                    println!(
                        " >> Case 3 blue ({}, {}) perp ({}, {})",
                        map_src_start, map_src_end, cur_src_start, cur_src_end
                    );
                    let start: u64 = val + (cur_src_start - map_src_start);
                    let end: u64 = start + (map_src_end - cur_src_start);
                    no_mutch.pop_front();
                    if start <= end {
                        println!(" >>> Pushing to    match ({}, {})", start, end);
                        source_map.insert(start, end);
                    }
                    if map_src_end + 1 <= cur_src_end {
                        println!(
                            " >>> Pushing to no-match ({}, {})",
                            map_src_end + 1,
                            cur_src_end
                        );
                        no_mutch.push_back((map_src_end + 1, cur_src_end));
                    }
                }
                else{
                    continue;
                }
                break;
            }
        }
        println!("!@#!@# no match at end: {:?}", no_mutch);
        for (start, end) in no_mutch.clone() {
            println!(" >> Clearing no-match");
            println!(" >>> Pushing to    match ({}, {})", start, end);
            source_map.insert(start, end);
        }
        println!(
            "Exited loop, source_map = {:?} is_empty = {}, source_map.len = {}, length = {}",
            source_map,
            no_mutch.is_empty(),
            source_map.len(),
            length
        );

    }
    return source_map;
}

fn main() {
    let mut source: HashMap<u64, u64> = get_seeds("seeds.txt");
    let file_names: Vec<&str> = vec![
        // "example_seed_to_soil.txt",
        // "example_soil-to-fertilizer.txt",
        // "example_fertilizer-to-water.txt",
        // "example_water_to_light.txt",
        // "example_light-to-temperature.txt",
        // "example_temperature-to-humidity.txt",
        // "example_humidity-to-location.txt",
        "seed_to_soil.txt",
        "soil-to-fertilizer.txt",
        "fertilizer-to-water.txt",
        "water-to-light map.txt",
        "light-to-temperature.txt",
        "temperature-to-humidity.txt",
        "humidity-to-location.txt",
    ];
    for file_name in file_names {
        let (fert_map, range_map) = create_map(file_name);
        source = create_source_lst(fert_map, range_map, source);
        println!("ended phase");
    }
    println!("{:?}, {}", source, source.len());

    // let behatzlacha = source.keys().clone().into_iter();
    println!("{:?}", source.keys().sorted());
    println!("{}", source.keys().into_iter().min().unwrap());
}
