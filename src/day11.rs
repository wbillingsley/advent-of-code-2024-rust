use std::{collections::HashMap, fs};
use num_bigint::BigUint;

fn read_input(file_path: String) -> String {
    println!("Reading input");

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}



fn part1() {
    let input = read_input("input.txt".to_string());

    // let numbers = input.split_whitespace().map(|str| {
    //     BigUint::parse_bytes(str.as_bytes(), 10).expect("Couldn't parse biguint")
    // }).collect::<Vec<_>>();

    let numbers = input.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

    fn parse_bi(s:&str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 10).expect("Not an integer")
    }

    fn expand(vec:&Vec<String>) -> Vec<String> {
        let mut next = Vec::new();
        for n in vec.into_iter() {
            let bi = parse_bi(n);
            if bi == BigUint::from(0 as u64) {
                next.push("1".to_string());
            } else if n.len() % 2 == 0 {
                let l = n.len();
                let (a, b) = n.split_at(l/2);
                next.push(parse_bi(a).to_string());
                next.push(parse_bi(b).to_string());
            } else {                
                let mult = bi * 2024 as u16;
                let str = mult.to_string();
                next.push(str);
            }

        }
        next
    }

    fn blink(count:u64, vec:Vec<String>) -> Vec<String> {
        let ans = (0 .. count).into_iter().fold(vec, |v, _| {
            // dbg!(&v);
            expand(&v) 
        });
        ans
    }

    dbg!(&numbers);

    dbg!(blink(25, numbers).len());


}


fn part2() {
    let input = read_input("input.txt".to_string());

    // This is a dirty cheat because the order of the numbers doesn't matter - we're just taking the count of stones
    // and each rule only requires this number, not any of its neighbours

    let numbers = input.split_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();

    fn parse_bi(s:&str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 10).expect("Not an integer")
    }

    let mut lookup_table = HashMap::<String, Vec<String>>::new();

    let mut expand_num = |s:&String| -> Vec<String> {

        match lookup_table.get_mut(s) {
            Some(v) => { v.clone() }
            _ => {
                let bi = parse_bi(&s);
                if bi == BigUint::from(0 as u64) {
                    let a = vec!["1".to_string()];
                    lookup_table.insert(s.clone(), a.clone());
                    a
                } else if s.len() % 2 == 0 {
                    let l = s.len();
                    let (a, b) = s.split_at(l/2);
                    let aa = parse_bi(a).to_string();
                    let bb = parse_bi(b).to_string();
                    let ans = vec![aa, bb];
                    lookup_table.insert(s.clone(), ans.clone());
                    ans
                } else {                
                    let mult = bi * 2024 as u16;
                    let str = mult.to_string();
                    let ans = vec![str];
                    lookup_table.insert(s.clone(), ans.clone());
                    ans
                }

            }

        }        
    };

    fn to_counts(vec:&Vec<String>) -> HashMap<String, u64> {
        let mut res = HashMap::new();
        for s in vec.into_iter() {
            match res.get(s) {
                Some(n) => { res.insert(s.clone(), n + 1 as u64); }
                None => { res.insert(s.clone(), 1 as u64); }
            }
        }
        res
    }

    let counts = to_counts(&numbers);

    let mut blink = |counts:HashMap<String, u64>| -> HashMap<String, u64> {
        let mut res = HashMap::new();

        for (k, v) in counts.into_iter() {
            let blunk = expand_num(&k);
            for s in blunk.into_iter() {
                match res.get(&s) {
                    Some(n) => { res.insert(s, n + v); }
                    None => { res.insert(s, v); }
                }
            }
        }
        res
    };

    let mut blink_n = |count:u64, counts:HashMap<String, u64>| -> HashMap<String, u64> {
        let mut res = counts;
        for i in 0 .. count {
            dbg!(i);
            res = blink(res);
        }
        res
    };

    dbg!(&numbers);

    let blunk = blink_n(75, counts);

    let ans = blunk.values().into_iter().sum::<u64>();



    dbg!(ans);
   

}

pub fn day11() {
    part1();
    part2();
}

