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
        // dbg!(&cap);
        cap[1].parse::<i64>().expect("Failed to parse {cap[1]}") * cap[2].parse::<i64>().expect("Failed to parse {cap[2]}")
    }).collect::<Vec<_>>();
    
    dbg!(mults.into_iter().sum::<i64>());


}


fn part2() {
    let input = read_input("input.txt".to_string());

    // In this slightly complex regex, "do"s will be in group 1, "don't"s in group 2, "mul(x,y)"s in group3, and the extracted x,y in groups 4 and 5
    let re = Regex::new(r"(do\(\))|(don\'t\(\))|(mul\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\))").unwrap();

    let mut on = true;
    
    let mults = re.captures_iter(input.as_str()).map(|cap| {
        dbg!(&cap);
        if cap.get(1).is_some() {
            on = true
        }
        if cap.get(2).is_some() {
            on = false
        }
        if on && cap.get(3).is_some() {
            cap[4].parse::<i64>().expect("Failed to parse {cap[1]}") * cap[5].parse::<i64>().expect("Failed to parse {cap[2]}")
        } else { 0 }
    }).collect::<Vec<_>>();
    
    dbg!(mults.into_iter().sum::<i64>());

}

pub fn day3() {
    part1();
    part2();
}

