use std::fs;
use regex::Regex;

fn read_input(file_path: String) -> Vec<String> {
    println!("Reading input");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let vec = contents.lines().map(|s| s.to_string()).collect();
    vec
}

#[derive(Debug, Clone, Copy)]
struct MachineState {
    instruction: usize,
    a: i64,
    b: i64,
    c: i64
}

impl MachineState {


    fn combo(&self, c:usize) -> i64 {
        match c {
            0 => { 0 },
            1 => { 1 },
            2 => { 2 },
            3 => { 3 },
            4 => { self.a },
            5 => { self.b },
            6 => { self.c },
            _ => { panic!("Invalid combo") }        
        }
    }

    fn inst(&self, prog:&Vec<usize> ) -> MachineState {
        let op = prog[self.instruction];
        match op {
            0 => {
                //adv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                MachineState { 
                    instruction: self.instruction + 2,
                    a: res,
                    ..*self
                }
            },

            1 => {
                // bxl
                let arg = prog[self.instruction + 1];
                let res = self.b as usize ^ arg;

                MachineState { 
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                }
            },

            2 => {
                // bst
                let comb = self.combo(prog[self.instruction + 1]);
                let res = comb % 8;

                MachineState {
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                }

            },

            3 => {
                // jnz
                if self.a == 0 {
                    MachineState {
                        instruction: self.instruction + 2,                    
                        ..*self
                    }
                } else {
                    MachineState {
                        instruction: prog[self.instruction + 1],                    
                        ..*self
                    }
                }
            },

            4 => {
                // bxc
                let res = self.b ^ self.c;

                MachineState {
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                }
            },

            5 => {
                // out
                let comb = self.combo(prog[self.instruction + 1]) % 8;
                print!("{comb},");

                MachineState {
                    instruction: self.instruction + 2,
                    ..*self
                }
            },

            6 => {
                //bdv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                MachineState { 
                    instruction: self.instruction + 2,
                    b: res,
                    ..*self
                }
            },

            7 => {
                //cdv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                MachineState { 
                    instruction: self.instruction + 2,
                    c: res,
                    ..*self
                }
            }


            _ => { panic!("Invalid opcode") }        
        }

    }
    
}



fn part1() {
    let input = read_input("input.txt".to_string());

    let re = Regex::new(r"([0-9]+)").unwrap();

    let a = re.captures(&input[0]).expect("No A register")[0].parse::<i64>().expect("Register not a number");
    let b = re.captures(&input[1]).expect("No B register")[0].parse::<i64>().expect("Register not a number");
    let c = re.captures(&input[2]).expect("No C register")[0].parse::<i64>().expect("Register not a number");
    let start = MachineState { a, b, c, instruction: 0 };

    let prog = re.captures_iter(&input[4]).map(|i| i[0].parse::<usize>().expect("Couldn't parse an instruction")).collect::<Vec<_>>();

    // dbg!(&start, prog);

    let mut cursor = start;
    while cursor.instruction < prog.len() {
        dbg!(cursor.instruction);
        cursor = cursor.inst(&prog);
    }



    

}

fn part2() {
    // not yet    

}

pub fn day17() {
    part1();
    part2();
}

