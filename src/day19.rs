use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn part1() {
    let input = read_input("input.txt".to_string());

    let towels = input[0].split(", ").collect::<Vec<_>>();

    let desired = &input[2..];

    let mut cands: HashSet<(&String, String)> = HashSet::from_iter(desired.iter().map(|s| (s, s.to_string())));
    let mut poss: HashSet<String> = HashSet::new();

    while !&cands.is_empty() {
        let mut next: HashSet<(&String, String)> = HashSet::new();
        for (orig, c) in cands.iter() {
            for &t in &towels {
                if c.starts_with(t) {
                    let remainder = &c[t.len()..];
                    if remainder.is_empty() {
                        poss.insert(orig.to_string());
                    } else {
                        next.insert((orig, remainder.to_string()));
                    }
                }
            }

        }

        cands = next;

    }

    dbg!(poss.len());



}



fn part2() {
    let input = read_input("input.txt".to_string());

    let towels = input[0].split(", ").collect::<Vec<_>>();

    let desired = &input[2..];


    let mut memo:HashMap<String, usize> = HashMap::new();

    fn count_arrangements(design:&str, towels:&Vec<&str>, memo: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() { 1 } else {
            towels.into_iter().map(|&t| {
                if design.starts_with(t) {

                    let rem = design[t.len()..].to_string();
                    
                    match memo.get(&rem) {
                        Some(&n) => {                             
                            n 
                        } 
                        _ => {
                            let c = count_arrangements(&design[t.len()..], towels, memo);
                            memo.insert(rem, c);
                            c
                        } 
                    }
                } else { 0 }
            }).sum::<usize>()
        }
    }

    let ans:usize = desired.iter().enumerate().map(|(i, design)| {
        dbg!(i);
        count_arrangements(design, &towels, &mut memo)
    }).sum();

    dbg!("Ans", ans);
    
    
}

pub fn day19() {
    part1();
    part2();
}

