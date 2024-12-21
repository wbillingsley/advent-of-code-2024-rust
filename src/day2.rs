use std::fs;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}


fn part1() {
    let input = read_input("input.txt".to_string());

    let grid = input.into_iter().map(|l| {
        let nums = l.split_whitespace().map(|s| {
            s.parse::<i32>().expect("Couldn't parse {s}")
        });
        nums.collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    fn safe_ascending(vec: &Vec<(i32, i32)>) -> bool {
        vec.into_iter().all(|(a, b)| {
            b > a && b - a <= 3    
        })
    }

    fn safe_descending(vec: &Vec<(i32, i32)>) -> bool {
        vec.into_iter().all(|(a, b)| {
            a > b && a - b <= 3    
        })
    }

    let safe = grid.into_iter().filter(|arr| {
        let pairs = arr.into_iter().zip(arr.into_iter().skip(1)).map(|(a, b)| (*a, *b)).collect::<Vec<_>>();

        safe_ascending(&pairs) || safe_descending(&pairs)
    }).count();

    dbg!(safe);



}


fn part2() {
    let input = read_input("input.txt".to_string());

    let grid = input.into_iter().map(|l| {
        let nums = l.split_whitespace().map(|s| {
            s.parse::<i32>().expect("Couldn't parse {s}")
        });
        nums.collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    fn safe_ascending(vec: &Vec<(i32, i32)>) -> bool {
        vec.into_iter().all(|(a, b)| {
            b > a && b - a <= 3    
        })
    }

    fn safe_descending(vec: &Vec<(i32, i32)>) -> bool {
        vec.into_iter().all(|(a, b)| {
            a > b && a - b <= 3    
        })
    }

    fn safe(vec: &Vec<(i32, i32)>) -> bool {
        safe_ascending(vec) || safe_descending(vec)
    }

    fn pairs(vec: &Vec<i32>) -> Vec<(i32, i32)> {
        vec.into_iter().zip(vec.into_iter().skip(1)).map(|(a, b)| (*a, *b)).collect::<Vec<_>>()
    }

    fn damped_safe(vec: &Vec<i32>) -> bool {
        let mut indices = (0 .. vec.len()).into_iter();

        indices.any(|x| {

            let removed = vec.into_iter().enumerate().filter_map(|(i, &e)| if i == x { None } else { Some(e) } ).collect();
            dbg!(&removed);
            safe(&pairs(&removed))
        })

    }

    let s = grid.into_iter().filter(|arr| {
        safe(&pairs(&arr)) || damped_safe(arr)
    }).count();

    dbg!(s);

}

pub fn day2() {
    part1();
    part2();
}

