use std::fs;
use std::ops;
use std::rc::Rc;
use priority_queue::PriorityQueue;
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

#[derive(Debug)]
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
    fn apply(v:T) -> Rc<List<T>> {
        Rc::new(
            List::Cons {
                item: v, 
                tail: Rc::new(List::Nil)
            }
        )
    }

    fn is_empty(&self) -> bool {
        match self {
            List::Nil => { true },
            _ => { false }
        }
    }

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
                if *d == *dir { -1 } else { -1001 }
            },
            None => { 
                if *dir == DIRECTIONS[1] { -1 } else { -1001 }
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

    let maze = FloorPlan::from(&input);

    fn find_path(maze:&FloorPlan) -> i64 {
        let mut paths = HashMap::new();
        let mut queue = PriorityQueue::new(); 

        let mut cursor = Path {
            directions: Rc::new(List::Nil), 
            end: maze.start,
            cost: 0
        };

        while {
            paths.insert(cursor.end, cursor.clone());

            for d in &DIRECTIONS {                
                if maze.can_move(&(cursor.end), &d) && !paths.contains_key(&(cursor.end + &d)) {
                    let pp = cursor.then(d);
                    let cost = pp.cost;
                    queue.push(pp, cost);
                }
            }

            !queue.is_empty() && !paths.contains_key(&maze.end)
        } {
            let (next, cost) = queue.pop().expect("Queue was empty");
            cursor = next;
        }

        let cheapest = paths.get(&maze.end).expect("No path found");

        let pic = maze.picture();
        println!("{pic}");
        dbg!(&cheapest.stringify_path());

        cheapest.cost
    }

    dbg!(find_path(&maze));





}

fn part2() {
    
    // not yet
}

pub fn day16() {
    part1();
    part2();
}

