use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

// Takes a grid of characters and returns a map from character to a set of locations
fn buckets(vec:&Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut res = HashMap::<char, HashSet<(usize, usize)>>::new();
    for (y, line) in vec.into_iter().enumerate() {
        for (x, c) in line.into_iter().enumerate() {
            match res.get_mut(c) {
                Some(v) => { v.insert((x, y)); }
                None => { res.insert(*c, HashSet::from([(x, y)])); }
            }
        }
    }
    res
}


// Takes a set of locations that may contain more than 1 contiguous region. Starting from the given position,
// uses flood fill to remove connected locations to a second set.
fn extract_region(grid_size:&(usize, usize), start:&(usize, usize), from:&HashSet<(usize, usize)>) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut extracted = HashSet::new();
    let mut old = from.clone();

    let (x, y) = &start;
    println!("extract from {x} {y}");

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mv = |from: &(usize, usize), by: &(i64, i64)| -> Option<(usize, usize)> {
        let (x_size, y_size) = grid_size;
        let (dx, dy) = by;
        let (x, y) = from;

        let xx = *x as i64 + dx;
        let yy = *y as i64 + dy;
        if xx >= 0 && (xx as usize) < *x_size && yy >= 0 && (yy as usize) < *y_size {
            Some((xx as usize, yy as usize))
        } else { None }
    };

    let mut queue = VecDeque::from([*start]);
    while let Some(loc) = queue.pop_front() {
        old.remove(&loc);
        extracted.insert(loc);

        // dbg!(queue.len());

        for d in &directions {
            if let Some(m) = mv(&loc, d) {
                if old.contains(&m) && !queue.contains(&m)  {
                    queue.push_back(m);
                } 
            }
        }
    }

    (extracted, old)
}

fn regionalise(grid_size:&(usize, usize), from:&HashSet<(usize, usize)>) -> Vec<HashSet<(usize, usize)>> {
    let mut regions = Vec::new();
    let mut cursor = from;
    let mut b;

    while let Some(el) = cursor.iter().nth(0) {
        let a;
        (a, b) = extract_region(grid_size, &el, cursor);
        cursor = &b;
        regions.push(a);
    }

    regions

}

fn perimeter(grid_size:&(usize, usize), from:&HashSet<(usize, usize)>) -> usize {
    let mut ans = 0;

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mv = |from: &(usize, usize), by: &(i64, i64)| -> Option<(usize, usize)> {
        let (x_size, y_size) = grid_size;
        let (dx, dy) = by;
        let (x, y) = from;

        let xx = *x as i64 + dx;
        let yy = *y as i64 + dy;
        if xx >= 0 && (xx as usize) < *x_size && yy >= 0 && (yy as usize) < *y_size {
            Some((xx as usize, yy as usize))
        } else { None }
    };

    for p in from {
        ans += 4;
        for d in &directions {
            if let Some(pp) = mv(p, d) {
                if from.contains(&pp) {
                    ans -= 1;
                }    
            }
        }

    }

    ans
}


fn part1() {
    let input = read_input("input.txt".to_string());
    let grid = &input.into_iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let grid_size = (grid[0].len(), grid.len());
    let buckets = buckets(&grid);

    let regions = buckets.into_iter().map(|(ch, b)| {
        println!("Processing {ch}");
        let rs = regionalise(&grid_size, &b);
        rs.into_iter().map(|r| {
            let area = r.len();
            let perim = perimeter(&grid_size, &r);
            println!(" {area} * {perim} ");
            area * perim
        }).sum::<usize>()

    }).sum::<usize>();

    dbg!(&regions);

}


fn part2() {

    // not yet

}

pub fn day12() {
    part1();
    part2();
}

