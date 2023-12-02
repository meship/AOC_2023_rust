use std::fs;


fn possible_per_select(input: Vec<u32>) -> bool{
    let RGB_BALLS_AMOUNT: Vec<u32> = vec![12, 13, 14];
    input[0] <= RGB_BALLS_AMOUNT[0] && input[1] <= RGB_BALLS_AMOUNT[1] 
    && input[2] <= RGB_BALLS_AMOUNT[2]
}

fn pre_process_selection(selections: Vec<&str>) -> bool{
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
    return possible_per_select(rgb_vec);

}

fn possible_game(game: &str) -> bool{
    let remove_game = game.split(":").nth(1).unwrap();
    let selections = remove_game.split(";");
    for select in selections.into_iter(){
        let select_vec: Vec<&str> = select.split(",").collect();
        if !pre_process_selection(select_vec) {
            return false;
        } 
    }
    true
    
}

fn main() {
    let file_path : &str = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut sum: u32 = 0;
    for (idx, line) in contents.lines().enumerate(){
        if possible_game(line){
            sum += idx as u32 + 1;
        }

    }

    println!("{}", sum); 
}
