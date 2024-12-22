use regex::Regex;
use std::fs;
use std::collections::HashSet;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn equation(vec:&String) -> (i64, Vec<i64>) {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let mut all = re.captures_iter(vec).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}"));

    let first = all.next().unwrap();
    let rest = all.collect::<Vec<_>>();
    
    (first, rest)

}


fn part1() {
    let input = read_input("input.txt".to_string());

    let equations = input.iter().map(|line| equation(&line));

    // Trying a sort of manual fold, just to explore the generics
    fn possibilities<'a>(mut iter: impl Iterator<Item = &'a i64>, last:HashSet<i64>) -> HashSet<i64> {
        if let Some(n) = iter.next() {
            let mut new_nums = HashSet::<i64>::new();
            if last.is_empty() {
                new_nums.insert(*n);
            } else {
                for x in last {
                    new_nums.insert(x + *n);
                    new_nums.insert(x * *n);
                }
            }
            possibilities(iter, new_nums)
        } else { last }
    };

    let possibly_true = equations.into_iter().filter(|(tot, nums)| {
        possibilities(nums.into_iter(), HashSet::new()).into_iter().any(|x| x == *tot)
    });
    
    dbg!(possibly_true.map(|(tot, _)| tot).sum::<i64>());

}


fn part2() {
    
    let input = read_input("input.txt".to_string());

    let equations = input.iter().map(|line| equation(&line));

    // Trying a sort of manual fold, just to explore the generics
    fn possibilities<'a>(mut iter: impl Iterator<Item = &'a i64>, last:HashSet<i64>) -> HashSet<i64> {
        if let Some(n) = iter.next() {
            let mut new_nums = HashSet::<i64>::new();
            if last.is_empty() {
                new_nums.insert(*n);
            } else {
                for x in last {
                    new_nums.insert(x + *n);
                    new_nums.insert(x * *n);

                    let mut concat = x.to_string();
                    concat.push_str(&n.to_string());
                    let c = concat.parse::<i64>().expect("Couldn't parse concatenation ");
                    new_nums.insert(c);
                }
            }
            possibilities(iter, new_nums)
        } else { last }
    };

    let possibly_true = equations.into_iter().filter(|(tot, nums)| {
        possibilities(nums.into_iter(), HashSet::new()).into_iter().any(|x| x == *tot)
    });
    
    dbg!(possibly_true.map(|(tot, _)| tot).sum::<i64>());
    

}

pub fn day7() {
    part1();
    part2();
}

