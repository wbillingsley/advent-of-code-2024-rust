use std::fs;
use std::ops;
use regex::Regex;
use std::rc::Rc;
use priority_queue::PriorityQueue;
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

impl Direction {
    fn inverse(&self) -> Vec2d {
        Vec2d{ x: -self.x, y: -self.y }
    }

    fn to_char(&self) -> char {
        if *self == DIRECTIONS[0] {
            '^'
        } else if *self == DIRECTIONS[1] {
            '>'
        } else if *self == DIRECTIONS[2] {
            'v'
        } else if *self == DIRECTIONS[3] {
            '<'
        } else { '?' }
    }
}

fn parse_command(ch: &char) -> Option<&'static Vec2d> {
    match ch {
        '^' => { Some(&DIRECTIONS[0]) },
        '>' => { Some(&DIRECTIONS[1]) },
        'v' => { Some(&DIRECTIONS[2]) },
        '<' => { Some(&DIRECTIONS[3]) },
        _ => None
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Square {
    Wall, Blank
}

impl Square {
    fn from(c:&char) -> Option<Square> {
        match c {
            '#' => { Some(Square::Wall) }
            '.' => { Some(Square::Blank) }
            _ => None
        }
    }

    fn to_char(&self) -> char {
        match self {
            Square::Wall => { '#' },
            Square::Blank => { '.' },
        }
    }
}

#[derive(Debug)]
struct FloorPlan {
    grid: Vec<Vec<Square>>,
    us: Vec2d
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

    fn corrupt(&mut self, p:&Vec2d) -> bool {
        if self.in_range(p) {
            self.grid[p.y as usize][p.x as usize] = Square::Wall;
            true
        } else { false }
    }

    fn new(w:usize, h:usize) -> FloorPlan {
        FloorPlan { 
            grid: (0..h).map(|y| {
                (0..w).map(|_| Square::Blank).collect::<Vec<_>>()
            }).collect::<Vec<_>>(),
            us: Vec2d { x:0, y:0 }
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


// Let's try out Rc to create an immutable singly linked list
// We need to reference count so we can share tails
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum List<T> {
    Nil, 
    Cons {
        item: T,
        tail: Rc<List<T>>
    }
}

impl <T> List<T> {

    fn head(&self) -> Option<&T> {
        match self {
            List::Nil => { None },
            List::Cons { item, tail: _ } => Some(item)
        }
    }

    fn fld<A> (&self, start:A, f: impl Fn(A, &T) -> A) -> A {
        match self {
            List::Nil => { start },
            List::Cons { item, tail } => { tail.fld(f(start, item), f) }
        }
    }

}

fn cons<T>(h: T, tail: Rc<List<T>>) -> Rc<List<T>> {
    Rc::new(
        List::Cons {
            item: h, 
            tail
        }
    )
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Path {
    directions: Rc<List<&'static Direction>>,
    end: Vec2d,
    cost: i64
}

impl Path {
    fn then(&self, dir: &'static Direction) -> Path {
        let directions = cons(dir, self.directions.clone());
        
        // We make cost negative so we can put it in a priority queue as-is
        let cost = self.cost + match self.directions.head() {
            Some(&d) => {
                if *d == *dir { -1 } else { -1 }
            },
            None => { 
                if *dir == DIRECTIONS[1] { -1 } else { -1 }
             }
        };

        let end = self.end + dir; 
        Path { directions, end, cost }
    }

    fn stringify_path(&self) -> String {
        let s = String::new();
        let ss = self.directions.fld(s, |mut ss, &d| {
            ss.push(d.to_char());
            ss
        });
        ss
    }

}



fn part1() {
    let input = read_input("input.txt".to_string());
    let re = Regex::new(r"([0-9]+)").unwrap();

    let coords:Vec<Vec2d> = input.iter().map(|line| {
        let arr = re.captures_iter(&line).map(|cap| cap[0].parse::<i64>().expect("Failed to parse coordinate")).collect::<Vec<_>>();
        Vec2d { x:arr[0], y: arr[1] }        
    }).collect::<Vec<_>>();


    let mut maze = FloorPlan::new(71,71);
    for p in coords.iter().take(1024) {
        maze.corrupt(&p);
    }

    let start = Vec2d { x:0, y: 0};
    let end = Vec2d { x:70, y: 70};

    fn flood_fill(maze:&FloorPlan, start:&Vec2d, end:&Vec2d) -> i64 {
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

        let cheapest = distances.get(end).expect("No path");

        let pic = maze.picture();
        println!("{pic}");
        dbg!(&cheapest);

        *cheapest

    }    

    dbg!(flood_fill(&maze, &start, &end));


}



fn part2() {
    // not yet
}

pub fn day18() {
    part1();
    part2();
}

