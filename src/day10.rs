use std::fs;
use std::collections::HashSet;


fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn num_grid(text:&Vec<String>) -> Vec<Vec<u64>> {
    text.into_iter().map(|line| {
        line.chars().into_iter().map(|c| { 
            c.to_digit(10).expect("Wasn't a digit from 0 to 9") as u64
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

fn trailheads(text:&Vec<String>) -> HashSet<(i64, i64)> {
    let mut ret = HashSet::new();
    for (y, line) in text.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {            
            if ch == '0' {
                ret.insert((x as i64, y as i64));
            }
        }
    }
    ret
}


fn part1() {
    let input = read_input("input.txt".to_string());
    let grid = num_grid(&input);
    let trailheads = trailheads(&input);

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let in_x_range = |x: i64| -> bool { x >= 0 && x < input[0].len() as i64 };
    let in_y_range = |y: i64| -> bool { y >= 0 && y < input.len() as i64 };


    let reachable = |start:(i64, i64)| -> HashSet<(i64, i64)> {        
        let mut locs = HashSet::from([start]);
        
        for num in 1..10 {
            let mut new_locs = HashSet::new();
            for (x, y) in locs {
                for (dx, dy) in &directions {
                    let xx = x + dx;
                    let yy = y + dy;
                    if in_x_range(xx) && in_y_range(yy) && grid[yy as usize][xx as usize] == num {
                        new_locs.insert((xx, yy));
                    }
                }                
            }
            locs = new_locs;
        }

        locs
    };

    let ans = trailheads.into_iter().map(|start| reachable(start).len()).sum::<usize>();


    dbg!(ans);



}


fn part2() {
    // Not yet

}

pub fn day10() {
    part1();
    part2();
}

