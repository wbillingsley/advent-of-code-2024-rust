use std::fs;
use std::ops;
use std::collections::VecDeque;

use std::collections::HashMap;



fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Vec2d { 
    x: i64,
    y: i64
}

impl ops::Add<&Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, _rhs: &Vec2d) -> Vec2d {
        Vec2d { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
}

type Direction = Vec2d;

const DIRECTIONS: [Direction; 4] = [ 
    Vec2d { x: 0, y: -1 }, 
    Vec2d { x: 1, y: 0 }, 
    Vec2d { x: 0, y: 1 }, 
    Vec2d { x: -1, y: 0 }
];


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Square {
    Wall, Blank, Start, End
}

impl Square {
    fn from(c:&char) -> Option<Square> {
        match c {
            '#' => { Some(Square::Wall) }
            '.' => { Some(Square::Blank) }
            'S' => { Some(Square::Start) }
            'E' => { Some(Square::End) }
            _ => None
        }
    }

    fn to_char(&self) -> char {
        match self {
            Square::Wall => { '#' },
            Square::Start => { 'S' },
            Square::Blank => { '.' },
            Square::End => { 'E' },
        }
    }
}

#[derive(Debug, Clone)]
struct FloorPlan {
    grid: Vec<Vec<Square>>,
    start: Vec2d,
    end: Vec2d
}

impl FloorPlan {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn in_range(&self, p:&Vec2d) -> bool {
        p.x >= 0 && p.x < self.width() as i64 && p.y >= 0 && p.y < self.height() as i64
    }

    fn find(&self, target:&Square) -> Option<Vec2d> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, sq) in row.into_iter().enumerate() {
                if *sq == *target {
                    return Some(Vec2d { x: x as i64, y: y as i64 });
                }
            }
        }
        None
    }

    fn from(plan_str:&[String]) -> FloorPlan {
        let _a = FloorPlan {
            grid: plan_str.into_iter().map(|line| {
                line.chars().filter_map(|c| Square::from(&c) ).collect::<Vec<_>>()
            }).collect::<Vec<_>>(),
            start: Vec2d { x: 0, y: 0 },
            end: Vec2d { x: 0, y: 0 },
        };

        FloorPlan {
            start: _a.find(&Square::Start).expect("No start in the floor plan"),
            end: _a.find(&Square::End).expect("No end in the floor plan"),
            .._a
        }
    } 

    fn square(&self, at:&Vec2d) -> Option<&Square> {
        if self.in_range(at) {
            Some(&self.grid[at.y as usize][at.x as usize])
        } else { None }
    }


    fn can_move(&self, p: &Vec2d, dir:&Direction) -> bool {
        let pp = *p + dir;
        if let Some(sq) = self.square(&pp) {
             *sq != Square::Wall
        } else { false }
    }


    fn picture(&self) -> String {
        let mut s = String::new();

        for row in self.grid.iter() {
            let mut line = String::new();

            for sq in row.iter() {
                line.push(sq.to_char());

            }

            line.push('\n');
            s.push_str(&line);
        }
        s.push('\n');
        s
    }

}




fn part1() {
    let input = read_input("input.txt".to_string());
    let maze = FloorPlan::from(&input);

    fn flood_fill(maze:&FloorPlan, start:&Vec2d, end:&Vec2d) -> HashMap<Vec2d, i64> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new(); 

        let mut cursor = (*start, 0 as i64);

        let pic = maze.picture();
        println!("{pic}");

        while {
            let (p, cost) = cursor;
            distances.insert(p, cost);        

            for d in &DIRECTIONS {                
                if maze.can_move(&p, &d) && !distances.contains_key(&(p + &d)) && !queue.contains(&(p + &d, cost + 1))  {                    
                    queue.push_back((p + &d, cost + 1));
                }
            }

            !queue.is_empty() && !distances.contains_key(end)
        } {
            let next = queue.pop_front().expect("Queue was empty");
            cursor = next;
        }

        distances

    }      

    let distances = flood_fill(&maze, &maze.start, &maze.end);

    let delta = 100;

    // A cheat has a path element and two steps away another path element with a later distance
    let num_cheats = distances.keys().map(|sq| {
        DIRECTIONS.iter().filter(|&d| {
            let two_away:Vec2d = *sq + d + d;
            let a = distances.get(sq).expect("Huh, a key had no get");
            if let Some(b) = distances.get(&two_away) {
                b - a > delta
            } else { false }
        }).count()
    }).sum::<usize>();

    dbg!(num_cheats);


}



fn part2() {
    let input = read_input("input.txt".to_string());
    let maze = FloorPlan::from(&input);

    fn flood_fill(maze:&FloorPlan, start:&Vec2d, end:&Vec2d) -> HashMap<Vec2d, i64> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new(); 

        let mut cursor = (*start, 0 as i64);

        let pic = maze.picture();
        println!("{pic}");

        while {
            let (p, cost) = cursor;
            distances.insert(p, cost);        

            for d in &DIRECTIONS {                
                if maze.can_move(&p, &d) && !distances.contains_key(&(p + &d)) && !queue.contains(&(p + &d, cost + 1))  {                    
                    queue.push_back((p + &d, cost + 1));
                }
            }

            !queue.is_empty() && !distances.contains_key(end)
        } {
            let next = queue.pop_front().expect("Queue was empty");
            cursor = next;
        }

        distances

    }      

    let distances = flood_fill(&maze, &maze.start, &maze.end);

    let delta = 100;
    let cheat_length = 20;

    // This is equivalent to counting the number of squares less that cheat_length manhattan distance away
    // that have a distance value >= than delta + manhattan distance.
    let num_cheats = distances.keys().map(|sq| {
        (-cheat_length..cheat_length + 1).map(|dy: i64| {
            let max_x:i64 = cheat_length - dy.abs();
            (-max_x..max_x+1).filter(|&dx| {
                let to = *sq + &Vec2d { x: dx, y: dy };                
                let a = distances.get(sq).expect("Huh, a key had no get");
                if let Some(b) = distances.get(&to) {                    
                    let saving = b - a - dx.abs() - dy.abs();
                    saving >= delta
                } else { false }
            }).count()
        }).sum::<usize>()
    }).sum::<usize>();

    dbg!(num_cheats);


}

pub fn day20() {
    part1();
    part2();
}

