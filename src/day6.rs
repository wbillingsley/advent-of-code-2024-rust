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

fn obstructions(text:&Vec<String>) -> ((i32, i32, usize), HashMap<(usize, usize), bool>) {
    let mut obstr = HashMap::new();
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
                obstr.insert((x, y), true);
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
        let mut next_pos = (0, 0);
    
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
            if obstr.contains_key(&(xx as usize, yy as usize)) {
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
    // not yet

}

pub fn day6() {
    part1();
    part2();
}

