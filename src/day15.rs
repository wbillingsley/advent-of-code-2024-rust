use std::fs;
use std::ops;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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
    Wall, Block, Robot, Blank
}

impl Square {
    fn from(c:&char) -> Option<Square> {
        match c {
            '#' => { Some(Square::Wall) }
            '.' => { Some(Square::Blank) }
            '@' => { Some(Square::Robot) }
            'O' => { Some(Square::Block) }
            _ => None
        }
    }

    fn to_char(&self) -> char {
        match self {
            Square::Wall => { '#' },
            Square::Robot => { '@' },
            Square::Blank => { '.' },
            Square::Block => { 'O' },
        }
    }
}

#[derive(Debug)]
struct FloorPlan {
    grid: Vec<Vec<Square>>,
    robot: Vec2d
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
            robot: Vec2d { x: 0, y: 0 }
        };

        FloorPlan {
            robot: _a.find_robot().expect("No robot in the floor plan"),
            .._a
        }
    } 

    fn square(&self, at:&Vec2d) -> Option<&Square> {
        if self.in_range(at) {
            Some(&self.grid[at.y as usize][at.x as usize])
        } else { None }
    }

    fn find_robot(&self) -> Option<Vec2d> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, sq) in row.into_iter().enumerate() {
                if *sq == Square::Robot {
                    return Some(Vec2d { x: x as i64, y: y as i64 });
                }
            }
        }
        None
    }

    fn first_blank(&self, dir:&Direction) -> Option<Vec2d> {
        let mut p = self.robot.clone() + dir;

        loop {
            if let Some(sq) = self.square(&p) {
                match sq {
                    Square::Wall => { return None }
                    Square::Blank => { return Some(p) }
                    _ => {
                        p = p + dir;
                        continue;
                    }
                }
            } else { return None }
        }
    }

    fn mv(&mut self, dir:&Direction) {
        if let Some(end) = self.first_blank(dir) {
            let start = self.robot.clone();
            let inv = dir.inverse();
            let mut cursor = end;
            while {
                cursor != start
            } {                            
                self.grid[cursor.y as usize][cursor.x as usize] = *self.square(&(cursor + &inv)).expect("Cursor was outside the grid");
                cursor = cursor + &inv;
            }

            self.grid[cursor.y as usize][cursor.x as usize] = Square::Blank;
            self.robot = self.robot + dir;
        }
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

    fn gps(&self) -> usize {
        let mut tot = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, sq) in row.iter().enumerate() {
                if *sq == Square::Block {
                    tot = tot + 100 * y + x;
                }

            } 

        }
        tot
    }



}

fn part1() {
    let input = read_input("input.txt".to_string());
    let mut split = input.split(|l| l.is_empty());
    let plan_str = split.next().expect("No floorplan found");
    let command_str = &split.next().expect("No commands found").iter().fold("".to_owned(), |a, bb| {
        let mut r = "".to_owned();
        r.push_str(&a);
        r.push_str(bb);
        r
    });


    let mut plan = FloorPlan::from(plan_str);
    let commands = command_str.chars().filter_map(|c| parse_command(&c) ).collect::<Vec<_>>();


    dbg!(&plan, &commands);

    commands.iter().for_each(|&dir| {
        plan.mv(dir);
        // let pic = plan.picture();
        // dbg!(dir);
        // println!("{pic}");
    });

    let pic = plan.picture();
    println!("{pic}");

    dbg!(plan.gps());

}


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum WideSquare {
    Wall, LBlock, RBlock, Robot, Blank
}

impl WideSquare {
    fn from(sq:Square) -> [WideSquare; 2] {
        match sq {
            Square::Wall => { [WideSquare::Wall, WideSquare::Wall] }
            Square::Block => { [WideSquare::LBlock, WideSquare::RBlock] }
            Square::Blank => { [WideSquare::Blank, WideSquare::Blank] }
            Square::Robot => { [WideSquare::Robot, WideSquare::Blank] }
        }
    }

    fn to_char(&self) -> char {
        match self {
            WideSquare::Wall => { '#' },
            WideSquare::Robot => { '@' },
            WideSquare::Blank => { '.' },
            WideSquare::LBlock => { '[' },
            WideSquare::RBlock => { ']' },
        }
    }
}

#[derive(Debug)]
struct WideFloorPlan {
    grid: Vec<Vec<WideSquare>>,
    robot: Vec2d
}

impl WideFloorPlan {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn in_range(&self, p:&Vec2d) -> bool {
        p.x >= 0 && p.x < self.width() as i64 && p.y >= 0 && p.y < self.height() as i64
    }

    fn from(fp:&FloorPlan) -> WideFloorPlan {
        let _a = WideFloorPlan {
            grid: fp.grid.iter().map(|row| {
                let mut r = Vec::new();
                row.into_iter().for_each(|c| {
                    WideSquare::from(*c).into_iter().for_each(|s| r.push(s));
                });
                r
            }).collect::<Vec<_>>(),
            robot: Vec2d { x: 0, y: 0 }
        };

        WideFloorPlan {
            robot: _a.find_robot().expect("No robot in the floor plan"),
            .._a
        }
    } 

    fn square(&self, at:&Vec2d) -> Option<&WideSquare> {
        if self.in_range(at) {
            Some(&self.grid[at.y as usize][at.x as usize])
        } else { None }
    }

    fn find_robot(&self) -> Option<Vec2d> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, sq) in row.into_iter().enumerate() {
                if *sq == WideSquare::Robot {
                    return Some(Vec2d { x: x as i64, y: y as i64 });
                }
            }
        }
        None
    }

    fn can_move(&self, p:&Vec2d, dir:&Direction) -> bool {
        let pp = *p + dir;

        match self.square(&pp) {
            Some(WideSquare::Blank) => { true },
            Some(WideSquare::LBlock) => {
                if *dir == DIRECTIONS[0] || *dir == DIRECTIONS[2] {
                    self.can_move(&pp, dir) && self.can_move(&(pp + &DIRECTIONS[1]), dir)
                } else {
                    self.can_move(&pp, dir)
                }
            }
            Some(WideSquare::RBlock) => {
                if *dir == DIRECTIONS[0] || *dir == DIRECTIONS[2] {
                    self.can_move(&pp, dir) && self.can_move(&(pp + &DIRECTIONS[3]), dir)
                } else {
                    self.can_move(&pp, dir)
                }
            }
            _ => { false }
        }
    }

    fn _mv(&mut self, p:&Vec2d, dir:&Direction, incoming:WideSquare) {
        if let Some(sq) = self.square(p) {
            let to = *p + dir;
            
            match sq {
                WideSquare::LBlock => {                
                    if *dir == DIRECTIONS[0] || *dir == DIRECTIONS[2] {
                        self._mv(&to, dir, *sq);
                        self.grid[p.y as usize][p.x as usize] = WideSquare::Blank; // So our partner doesn't make us move again
                        let other = *p + &DIRECTIONS[1];
                        // dbg!("left", &p, other);
                        self._mv(&other, dir, WideSquare::Blank);
                        self.grid[p.y as usize][p.x as usize] = incoming;
                    } else {
                        self._mv(&to, dir, *sq);
                        self.grid[p.y as usize][p.x as usize] = incoming;
                    }
                },
                WideSquare::RBlock => {                
                    if *dir == DIRECTIONS[0] || *dir == DIRECTIONS[2] {
                        self._mv(&to, dir, *sq);
                        self.grid[p.y as usize][p.x as usize] = WideSquare::Blank;  // So our partner doesn't make us move again
                        let other = *p + &DIRECTIONS[3];
                        // dbg!("right", &p, other);
                        self._mv(&other, dir, WideSquare::Blank);
                        self.grid[p.y as usize][p.x as usize] = incoming;
                    } else {
                        self._mv(&to, dir, *sq);
                        self.grid[p.y as usize][p.x as usize] = incoming;
                    }
                },
                WideSquare::Blank => {
                    self.grid[p.y as usize][p.x as usize] = incoming;
                },
                WideSquare::Wall => {
                    
                },
                _ => {
                    self._mv(&to, dir, *sq);
                    self.grid[p.y as usize][p.x as usize] = incoming;
                }
            };
            
    
    }


        
    }

    fn mv(&mut self, dir:&Direction) {
        if self.can_move(&self.robot, dir) {
            self._mv(&self.robot.clone(), dir, WideSquare::Blank);
            self.robot = self.robot + dir;
        } 
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

    fn gps(&self) -> usize {
        let mut tot = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, sq) in row.iter().enumerate() {
                if *sq == WideSquare::LBlock {
                    tot = tot + 100 * y + x;
                }

            } 

        }
        tot
    }



}

fn part2() {
    
    let input = read_input("input.txt".to_string());
    let mut split = input.split(|l| l.is_empty());
    let plan_str = split.next().expect("No floorplan found");
    let command_str = &split.next().expect("No commands found").iter().fold("".to_owned(), |a, bb| {
        let mut r = "".to_owned();
        r.push_str(&a);
        r.push_str(bb);
        r
    });


    let s_plan = FloorPlan::from(plan_str);
    let mut plan = WideFloorPlan::from(&s_plan);
    let commands = command_str.chars().filter_map(|c| parse_command(&c) ).collect::<Vec<_>>();


    // dbg!(&plan, &commands);

    commands.iter().for_each(|&dir| {
        plan.mv(dir);
        // let pic = plan.picture();
        // dbg!(dir);
        // println!("{pic}");
    });

    let pic = plan.picture();
    println!("{pic}");

    dbg!(plan.gps());
}

pub fn day15() {
    part1();
    part2();
}

