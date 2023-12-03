use std::fs;

fn pre_process_selection(selections: Vec<&str>) -> Vec<u32>{
    let mut rgb_vec: Vec<u32> = vec![0, 0, 0];
    for selection in selections.iter(){
        let new_selection: &str = selection.strip_prefix(" ").unwrap();
        if (new_selection).contains("red"){
           rgb_vec[0] = ((new_selection.split(" ").nth(0).unwrap()).to_string()).parse::<u32>().unwrap()
        } else if (*selection).contains("green"){
            rgb_vec[1] = ((new_selection.split(" ").nth(0).unwrap()).to_string()).parse::<u32>().unwrap()
        } else {
            rgb_vec[2] = ((new_selection.split(" ").nth(0).unwrap()).to_string()).parse::<u32>().unwrap()
        }
    }
    return rgb_vec;

}


fn find_max_game_vec(rgb_selections: Vec<Vec<u32>>) -> u32{
    let red_vec: Vec<u32> = rgb_selections.iter().map(|voc| voc[0]).collect();
    let max_red: u32 = red_vec.iter().cloned().max().unwrap();
    let green_vec: Vec<u32> = rgb_selections.iter().map(|voc| voc[1]).collect();
    let max_green: u32 = green_vec.iter().cloned().max().unwrap();
    let blue_vec: Vec<u32> = rgb_selections.iter().map(|voc| voc[2]).collect();
    let max_blue: u32 = blue_vec.iter().cloned().max().unwrap();
    max_red * max_blue * max_green


}
fn find_power_for_game(game: &str) -> u32{
    let remove_game = game.split(":").nth(1).unwrap();
    let selections = remove_game.split(";");
    let mut rgb_selections: Vec<Vec<u32>> = Vec::new();
    for select in selections.into_iter(){
        let select_vec: Vec<&str> = select.split(",").collect();
        rgb_selections.push(pre_process_selection(select_vec))
    }

    return find_max_game_vec(rgb_selections);

    
}

fn main() {
    let file_path : &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: u32 = 0;
    for line in contents.lines(){
        sum += find_power_for_game(line);

    }

    println!("{}", sum); 
}
