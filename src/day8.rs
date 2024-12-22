use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn read_locations(plan:&Vec<String>) -> HashMap<char, Vec<(i64, i64)>> {
    let mut results = HashMap::<char, Vec<(i64, i64)>>::new();

    for (y, line) in plan.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                if results.contains_key(&c) {
                    results.get_mut(&c).unwrap().push((x as i64, y as i64));

                } else {
                    results.insert(c, vec![(x as i64, y as i64)]);
                }
            }
        }
    }

    results
}



fn part1() {
    let input = read_input("input.txt".to_string());
    
    let x_in_range = |x: i64| -> bool { x >= 0 && x < input[0].len() as i64 };
    let y_in_range = |y: i64| -> bool { y >= 0 && y < input.len() as i64 };
    let in_range = |loc: (i64, i64)| -> bool { 
        let (x, y) = loc;
        x_in_range(x) && y_in_range(y)
    };

    let project = |a: &(i64, i64), b: &(i64, i64)| -> (i64, i64) {
        let (x, y) = a;
        let (xx, yy) = b;
        let dx = xx - x;
        let dy = yy - y;
        (xx + dx, yy + dy)
    };

    let locations = read_locations(&input);

    let mut antinodes = HashSet::<(i64, i64)>::new();

    locations.into_iter().for_each(|(_, locs)| {
        for p in &locs {
            for pp in &locs {
                if p != pp {
                    let proj = project(p, pp);
                    if in_range(proj) {
                        antinodes.insert(proj);
                    }
                }
            }
        }
    });

    dbg!(&antinodes.len());


}


fn part2() {
    // Not yet

}

pub fn day8() {
    part1();
    part2();
}

