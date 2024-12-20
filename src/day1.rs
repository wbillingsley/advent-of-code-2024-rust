use std::fs;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn parse_l(v: &Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().map(|vec| vec[0]).collect::<Vec<_>>()
} 

fn parse_r(v: &Vec<Vec<i32>>) -> Vec<i32> {
    v.into_iter().map(|vec| vec[1]).collect::<Vec<_>>()
} 

fn part1() {
    let input = read_input("input.txt".to_string());

    let as_pairs = input.into_iter().map(|l| {
        let nums = l.split_whitespace().map(|s| {
            s.parse::<i32>().expect("Couldn't parse {s}")
        });
        nums.collect::<Vec<_>>()
    }).collect();

    // let left = &as_pairs.map(|vec| vec[0]).collect::<Vec<_>>();
    // let right = as_pairs.map(|vec| vec[1]).collect::<Vec<_>>();
        
    let mut left = parse_l(&as_pairs);
    left.sort();
    let mut right = parse_r(&as_pairs);
    right.sort();

    let _ = dbg!(&left);

    let zipped = left.into_iter().zip(right.into_iter()).map(|(a, b)| (a - b).abs());
    let sum = zipped.sum::<i32>();
    let _ = dbg!(sum);

}


fn part2() {
    let input = read_input("input.txt".to_string());

    let as_pairs = input.into_iter().map(|l| {
        let nums = l.split_whitespace().map(|s| {
            s.parse::<i32>().expect("Couldn't parse {s}")
        });
        nums.collect::<Vec<_>>()
    }).collect();
        
    let left = parse_l(&as_pairs);
    let right = &parse_r(&as_pairs);
    
    let similarities = left.into_iter().map(|x| { (x as i64) * right.into_iter().filter(|&v| *v == x).count() as i64 });

    let sum = similarities.sum::<i64>();
    let _ = dbg!(sum);
}

pub fn day1() {
    part1();
    part2();
}

//
// let mut s = String::new();
// std::io::stdin().read_line(&mut s).expect("read_line error");

// let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
// match (parts.next(), parts.next()) {
//     (Some(Ok(a)), Some(Ok(b))) => {
//         // a and b are i32
//     }
//     // handle other problems: not enough numbers, numbers are invalid, etc
//     _ => {}  // ignore invalid input
// }