use regex::Regex;
use std::fs;
use std::cmp;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}


fn part1() {
    let input = read_input("input.txt".to_string());
    let re = Regex::new(r"([0-9]+)").unwrap();

    // The pattern is three lines of info, 1 lone of blank
    let mins = input.chunks(4).map(|block| {
        let aa = re.captures_iter(&block[0]).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}")).collect::<Vec<_>>(); // button a
        let bb = re.captures_iter(&block[1]).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}")).collect::<Vec<_>>(); // button b
        let cc = re.captures_iter(&block[2]).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}")).collect::<Vec<_>>(); // prize

        let ax = aa[0];
        let ay = aa[1];
        let bx = bb[0];
        let by = bb[1];
        let tx = cc[0];
        let ty = cc[1];

        let max_a = cmp::min(cmp::min(tx / ax, ty / ay), 100);
        let max_b = cmp::min(cmp::min(tx / bx, ty / by), 100);

        let mut possibilities = Vec::<i64>::new();
        for a in 0..max_a + 1 {
            for b in 0..max_b + 1 {
                if ((a * ax) + (b * bx) == tx) && ((a * ay) + (b * by) == ty) {
                    possibilities.push(3 * a + b);
                }

            }
        }

        possibilities.into_iter().min()
    });

    let mut tot = 0 as i64;
    for opt in mins {
        dbg!(&opt);
        if let Some(v) = opt {
            tot += v;
        }
    }


    dbg!(tot);

}


fn part2() {
    
    // not yet

}

pub fn day13() {
    part1();
    part2();
}
