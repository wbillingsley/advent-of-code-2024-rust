use std::fs;
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

    fn parseBi(s:&str) -> BigUint {
        BigUint::parse_bytes(s.as_bytes(), 10).expect("Not an integer")
    }

    fn expand(vec:&Vec<String>) -> Vec<String> {
        let mut next = Vec::new();
        for n in vec.into_iter() {
            let bi = parseBi(n);
            if bi == BigUint::from(0 as u64) {
                next.push("1".to_string());
            } else if n.len() % 2 == 0 {
                let l = n.len();
                let (a, b) = n.split_at(l/2);
                next.push(parseBi(a).to_string());
                next.push(parseBi(b).to_string());
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
    // not yet
   

}

pub fn day11() {
    part1();
    part2();
}

