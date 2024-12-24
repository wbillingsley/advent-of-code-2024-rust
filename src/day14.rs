use regex::Regex;
use std::fs;
use std::ops;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

#[derive(Debug)]
struct Vec2d { 
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Robot {
    p: Vec2d,
    v: Vec2d
}

struct Quad {
    tl: Vec2d,
    br: Vec2d
}

impl Robot {
    fn step(&mut self, dimensions:&Vec2d) {
        let x = (self.p.x + self.v.x).rem_euclid(dimensions.x);
        let y = (self.p.y + self.v.y).rem_euclid(dimensions.y);

        self.p = Vec2d { x, y }
    }

    fn is_in(&self, quad:&Quad) -> bool {
        self.p.x >= quad.tl.x && self.p.y >= quad.tl.y && self.p.x < quad.br.x && self.p.y < quad.br.y
    }
}

impl ops::Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, _rhs: Vec2d) -> Vec2d {
        Vec2d { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

fn part1() {
    let input = read_input("input.txt".to_string());
    let re = Regex::new(r"(-?[0-9]+)").unwrap();

    let w = 101 as i64;
    let h = 103 as i64;

    let mut robots = input.into_iter().map(|line| {
        let nums = re.captures_iter(&line).map(|cap| cap[0].parse::<i64>().expect("Failed to parse {num}")).collect::<Vec<_>>(); 
        Robot {
            p: Vec2d { x: nums[0], y: nums[1] },
            v: Vec2d { x: nums[2], y: nums[3] }
        }
    }).collect::<Vec<Robot>>();

    for _ in 0..100 {
        for r in robots.iter_mut() {
            r.step(&Vec2d{ x: w, y: h });
        }
    }

    let quads = [
        Quad { tl: Vec2d { x:0, y: 0 }, br: Vec2d { x: w/2, y: h/2 }},
        Quad { tl: Vec2d { x:w/2 + 1, y: 0 }, br: Vec2d { x: w, y: h/2 }},
        Quad { tl: Vec2d { x:0, y: h/2 + 1 }, br: Vec2d { x: w/2, y: h }},
        Quad { tl: Vec2d { x:w/2 + 1, y: h/2 + 1 }, br: Vec2d { x: w, y: h }}
    ];

    dbg!(&robots);

    let contents = quads.map(|q| {
        robots.iter().filter(|r| r.is_in(&q)).count()
    });

    let ans = contents.iter().fold(1 as usize, |a, b| a * b);

    dbg!(ans);

}


fn part2() {
    
    // not yet

}

pub fn day14() {
    part1();
    part2();
}
