use regex::Regex;
use std::fs;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn rules(vec:&Vec<String>) -> Vec<(i64, i64)> {
    let rblock = vec.into_iter().take_while(|&line| !line.is_empty());
    let re = Regex::new(r"([0-9]+)\|([0-9]*)").unwrap();

    rblock.map(|line| {
        let cap = re.captures(line).unwrap();
        (cap[1].parse::<i64>().expect("Failed to parse {cap[1]}"), cap[2].parse::<i64>().expect("Failed to parse {cap[2]}"))
    }).collect::<Vec<_>>()
}

fn pages(vec:&Vec<String>) -> Vec<Vec<i64>> {
    let rblock = vec.into_iter().skip_while(|&line| !line.is_empty()).skip(1);
    let re = Regex::new(r"([0-9]+)").unwrap();

    rblock.map(|line| {
        re.captures_iter(line).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}")).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}


fn part1() {
    let input = read_input("input.txt".to_string());

    let rules = &rules(&input);
    let pages = pages(&input);

    // We meet the rule so long as we don't have both a and b, with b before a (i.e. we just scan and if we hit a check we didn't have b first)
    fn meets_rule(a:i64, b:i64, pp: &Vec<i64>) -> bool {
        let mut found_b = false;
        let violation = pp.into_iter().any (|&x| {
            if x == b { found_b = true };
            x == a && found_b
        });

        !violation
    }
    
    let good_updates = pages.into_iter().filter(|pps| {
        rules.into_iter().all(|(a, b)| meets_rule(*a, *b, pps))
    });

    let middle_pages = good_updates.map(|vec| {
        vec[vec.len() / 2]
    });
    
    dbg!(middle_pages.sum::<i64>());


}


fn part2() {
    let input = read_input("input.txt".to_string());

    let rules = &rules(&input);
    let pages = pages(&input);

    // We meet the rule so long as we don't have both a and b, with b before a (i.e. we just scan and if we hit a check we didn't have b first)
    fn fails_rule(a:i64, b:i64, pp: &Vec<i64>) -> bool {
        let mut found_b = false;
        let violation = pp.into_iter().any (|&x| {
            if x == b { found_b = true };
            x == a && found_b
        });

        violation
    }
    
    let bad_updates = pages.into_iter().filter(|pps| {
        rules.into_iter().any(|(a, b)| fails_rule(*a, *b, pps))
    }).collect::<Vec<_>>();

    let resort = |pps: &Vec<i64>| -> Vec<i64> {
        // For this to be possible, the rules containing numbers that appear must provide a complete ordering
        let filtered_rules = &rules.into_iter().filter(|(a, b)| pps.into_iter().any(|x| x == a) && pps.into_iter().any(|x| x == b) ).collect::<Vec<_>>();

        // A quick look at the filtered rules suggests we have more rules left than necessary. So, let's try this as a sort_by
        // dbg![filtered_rules];
        let mut sorted = pps.clone();
        sorted.sort_by(|a, b| {
            if filtered_rules.into_iter().any(|(aa, bb)| {
                a == aa && b == bb 
            }) { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        });

        sorted

        // vec![]
    };

    dbg!(bad_updates.into_iter().map(|pps| {
        let r = resort(&pps);
        r[r.len() / 2]
    }).sum::<i64>());
    

}

pub fn day5() {
    part1();
    part2();
}

