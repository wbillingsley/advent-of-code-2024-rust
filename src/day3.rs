use regex::Regex;
use std::fs;

fn read_input(file_path: String) -> String {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}


fn part1() {
    let input = read_input("input.txt".to_string());

    let re = Regex::new(r"mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\)").unwrap();
    
    let mults = re.captures_iter(input.as_str()).map(|cap| {
        dbg!(&cap);
        cap[1].parse::<i64>().expect("Failed to parse {cap[1]}") * cap[2].parse::<i64>().expect("Failed to parse {cap[1]}")
    }).collect::<Vec<_>>();
    
    dbg!(mults.into_iter().sum::<i64>());


}


fn part2() {
    // Not yet

}

pub fn day3() {
    part1();
    part2();
}

