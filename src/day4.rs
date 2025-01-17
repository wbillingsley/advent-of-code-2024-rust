use std::fs;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}


fn part1() {
    let directions = vec![
        vec![-1, -1], vec![0, -1], vec![1, -1],
        vec![-1,  0],              vec![1,  0],
        vec![-1,  1], vec![0,  1], vec![1,  1]
    ];

    let input = read_input("input.txt".to_string());

    let numy = input.len() as i64;
    let numx = input[0].len() as i64;

    let in_x_range = |x: i64| -> bool { x >= 0 && x < numx };
    let in_y_range = |y: i64| -> bool { y >= 0 && y < numy };

    let mut count = 0;

    let str = "XMAS";

    for y in 0 .. numy {
        for x in 0 .. numx {
            for d in &directions {
                if (0 .. str.len()).all(|i | {
                    let dx = d[0] * i as i64;
                    let dy = d[1] * i as i64;
                    let xx = x + dx;
                    let yy = y + dy;
                    in_x_range(xx) && in_y_range(yy) && input[yy as usize].chars().nth(xx as usize) == str.chars().nth(i)
                }) {
                    count = count + 1
                }
            }

        }
    }

    dbg!(count);



}


fn part2() {
    let input = read_input("input.txt".to_string());

    let numy = input.len();
    let numx = input[0].len();

    let in_x_range = |x: usize| -> bool { x < numx };
    let in_y_range = |y: usize| -> bool { y < numy };

    let mut count = 0;

    for y in 1 .. numy {
        for x in 1 .. numx {
            
            let left = x - 1;
            let right = x + 1;
            let top = y - 1;
            let bottom = y + 1;

            if input[y].chars().nth(x) == Some('A') &&
                in_y_range(bottom) &&
                in_x_range(right) &&
                ((input[top].chars().nth(left) == Some('M') && input[bottom].chars().nth(right) == Some('S')) || (input[top].chars().nth(left) == Some('S') && input[bottom].chars().nth(right) == Some('M'))) && 
                ((input[top].chars().nth(right) == Some('M') && input[bottom].chars().nth(left) == Some('S')) || (input[top].chars().nth(right) == Some('S') && input[bottom].chars().nth(left) == Some('M'))) 
            {
                count = count + 1
            }
            
    
        }
    }

    dbg!(count);

}

pub fn day4() {
    part1();
    part2();
}

