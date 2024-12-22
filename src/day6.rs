use std::fs;
use std::collections::HashSet;


fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

fn obstructions(text:&Vec<String>) -> ((i32, i32, usize), HashSet<(usize, usize)>) {
    let mut obstr = HashSet::new();
    let mut start = (-1, -1, 0); // This'll get overwritten
    for (y, line) in text.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '^' {
                start = (x as i32, y as i32, 0);
            }
            if ch == '>' {
                start = (x as i32, y as i32, 1);
            }
            if ch == 'V' {
                start = (x as i32, y as i32, 2);
            }
            if ch == '<' {
                start = (x as i32, y as i32, 3);
            }
            if ch == '#' {
                obstr.insert((x, y));
            }
        }
    }
    (start, obstr)
}


fn part1() {
    let input = read_input("input.txt".to_string());

    let (start, obstr) = obstructions(&input);

    let mut state = start;
    let mut visited = HashSet::<(i32, i32)>::new();

    let in_x_range = |x: i32| -> bool { x >= 0 && x < input[0].len() as i32 };
    let in_y_range = |y: i32| -> bool { y >= 0 && y < input.len() as i32 };

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    {
        let mut next_pos;
    
        while {
            let (x, y, dir) = state;

            visited.insert((x, y));
            
            let (dx, dy) = directions[dir];
            let xx = x + dx;
            let yy = y + dy;
            next_pos = (xx, yy); // Next position if we were to had in the same direction

            in_x_range(xx) && in_y_range(yy) // End if exited the map
        } /*do*/ {
            // Turn if we need to
            let (x, y, dir) = state;
            let (xx, yy) = next_pos;
            if obstr.contains(&(xx as usize, yy as usize)) {
                state = (x, y, (dir + 1) % 4);
                println!("Turning at {xx} {yy} {dir}");
            } else {
                state = (xx, yy, dir);
            }
        }
    }

    dbg!(visited.len());



}


fn part2() {
    // We have to add an obstruction to put the guard into a visited square.
    // Notice -
    // - This must be a visited square along the guard's route
    // - Continuing the journey must reach a previously visited square (in the same direction) before it exits

    let input = read_input("input.txt".to_string());

    let (start, obstr) = obstructions(&input);
    let (x0, y0, _) = start;

    fn walk(start: &(i32, i32, usize), obstacles: &HashSet<(usize, usize)>, seen: &HashSet<(i32, i32, usize)>, lenx:usize, leny:usize) -> (bool, HashSet<(i32, i32, usize)>) {
        let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

        let in_x_range = |x: i32| -> bool { x >= 0 && x < lenx as i32 };
        let in_y_range = |y: i32| -> bool { y >= 0 && y < leny as i32 };
    
        let mut loc_seen = seen.clone();
        let mut state = start.clone();
        let mut next_pos;

        let mut looped = false;
    
        while {
        
            if loc_seen.contains(&state) {
                looped = true;
            } else {
                loc_seen.insert(state);
            }

            let (x, y, dir) = state;
            
            let (dx, dy) = directions[dir];
            let xx = x + dx;
            let yy = y + dy;
            next_pos = (xx, yy); // Next position if we were to had in the same direction

            !looped && in_x_range(xx) && in_y_range(yy) // End if looped or exited
        } /*do*/ {
            // Turn if we need to
            let (x, y, dir) = state;
            let (xx, yy) = next_pos;
            if obstacles.contains(&(xx as usize, yy as usize)) {
                state = (x, y, (dir + 1) % 4);
                // println!("Turning at {xx} {yy} {dir}");
            } else {
                state = (xx, yy, dir);
            }
        }

        (looped, loc_seen)
    }

    let (_, visited) = walk(&start, &obstr, &HashSet::<(i32, i32, usize)>::new(), input[0].len(), input.len());
    let squares = visited.iter().map(|(x, y, _)| (x, y)).collect::<HashSet<_>>(); // Should be the same as part 1


    let test_obstacle = |loc:(usize, usize)| -> bool {
        let mut altered_obstacles = obstr.clone();
        altered_obstacles.insert(loc);

        let (result, _) = walk(&start, &altered_obstacles, &HashSet::<(i32, i32, usize)>::new(), input[0].len(), input.len());
        result
    };

    let ans = squares.iter().filter(|&loc| {
        let (&x, &y) = *loc;
        (x, y) != (x0, y0) && test_obstacle((x as usize, y as usize))
    }).count();



    dbg!(ans);

}

pub fn day6() {
    part1();
    part2();
}

