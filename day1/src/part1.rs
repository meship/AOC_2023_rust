fn find_ref_value(input: &mut String) -> i32{
    find_first_num(input, false)* 10 + find_first_num(input, true)
}
fn find_first_num(input: &mut String, reverse: bool) -> i32{
    let iterate: Box<dyn Iterator<Item = char>> = if reverse {
        Box::new(input.chars().rev())
    } else {
        Box::new(input.chars())
    };
    for c in iterate {
        if c.is_numeric(){
            return (c.to_string()).parse::<i32>().unwrap();
        }
    }
    return 0;
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