use std::fs;
use regex::Regex;
use std::collections::HashSet;


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

    fn inst(&self, prog:&Vec<usize> ) -> (Option<i64>, MachineState) {
        let op = prog[self.instruction];
        match op {
            0 => {
                //adv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                (None, MachineState { 
                    instruction: self.instruction + 2,
                    a: res,
                    ..*self
                })
            },

            1 => {
                // bxl
                let arg = prog[self.instruction + 1];
                let res = self.b as usize ^ arg;

                (None, MachineState { 
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                })
            },

            2 => {
                // bst
                let comb = self.combo(prog[self.instruction + 1]);
                let res = comb % 8;

                (None, MachineState {
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                })

            },

            3 => {
                // jnz
                if self.a == 0 {
                    (None, MachineState {
                        instruction: self.instruction + 2,                    
                        ..*self
                    })
                } else {
                    (None, MachineState {
                        instruction: prog[self.instruction + 1],                    
                        ..*self
                    })
                }
            },

            4 => {
                // bxc
                let res = self.b ^ self.c;

                (None, MachineState {
                    instruction: self.instruction + 2,
                    b: res as i64,
                    ..*self
                })
            },

            5 => {
                // out
                let comb = self.combo(prog[self.instruction + 1]) % 8;
                // print!("{comb},");

                (Some(comb), MachineState {
                    instruction: self.instruction + 2,
                    ..*self
                })
            },

            6 => {
                //bdv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                (None, MachineState { 
                    instruction: self.instruction + 2,
                    b: res,
                    ..*self
                })
            },

            7 => {
                //cdv
                let num = self.a;
                let denom = (2 as i64).pow(self.combo(prog[self.instruction + 1]) as u32);
                let res = num / denom;
                
                (None, MachineState { 
                    instruction: self.instruction + 2,
                    c: res,
                    ..*self
                })
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
        let opt;
        // dbg!(cursor.instruction);
        (opt, cursor) = cursor.inst(&prog);
        
        if let Some(n) = opt {
            print!("{n},");
        }
    }



    

}

fn part2() {

    // To work this one out, you have to reverse engineer YOUR program. (An arbitrary program might not terminate, might move to an odd instruction index, etc)
    // 
    // 2,4, 1,7, 7,5, 1,7, 4,6, 0,3, 5,5, 3,0 (spaces inserted for ease of reading)

    // 3,0 at end loops to the start.
    // there is one 5 (out) per loop.

    // a is shifted right 3 bits per loop. So, a has at least 48 bits (probably - unless b unwinding against zeros proks for the last few numbers)

    // Figure out the first loop 
    // 2,4  b = lowest 3 bits of A
    // 1,7 b = ~b
    // 7,5 c = a/b
    // 1,7 b = ~b
    // 4,6 b = b xor c
    // 0,3 a = a >> 3
    // 5,5 out(b)
    // 3,0 loop

    // So if our first number is 2, we should be able to figure out the lowest bits of A.

    // take lowest 3 bits
    // invert them
    // shift the num right that many bits
    // xor

    // i.e. "the next 3 bits up the chain, shifted right by these 3 bits inverse, differ by 2, 4, 1, etc."

    
    // fn test(a:usize, target:usize) -> bool {
    //     let b = a % 8;
    //     let c = a >> ((!b) % 8);
    //     let xored = (b ^ c) % 8;
    //     xored == target
    // }

    type Candidate = (usize, usize);

    // We can only join parts of candidates if they agree on any overlap in their masks
    fn join(a:&Candidate, b:&Candidate) -> Option<Candidate>  {
        let (aa, amask) = a;
        let (bb, bmask) = b;

        let cc = aa | bb;
        let cmask = amask | bmask;

        if (cc & amask == *aa) && (cc & bmask == *bb) {
            Some((cc, cmask))
        } else { None }
    }

    // Let' try generating possibilites and hope the constraints narrow it down

    fn generate_candidates_at(incoming:&Candidate, lshift:usize, xor:usize) -> HashSet<Candidate> {

        let parts = (0..8).filter_map(|oct| {
            let shift = !oct & 7;
            let low_bits = oct << lshift;
            let low_mask = 7 << lshift;
            let high_bits = (oct ^ xor) << (lshift + shift);
            let high_mask = 7 << (lshift + shift); // Sets the lowest 3 bits plus the comparison to whichever higher bits
            
            join(&(low_bits, low_mask), &(high_bits, high_mask)) 
        });

        parts.filter_map(|c| join(&incoming, &c)).collect::<HashSet<Candidate>>()
    }

    let prog: Vec<usize> = vec![2,4, 1,7, 7,5, 1,7, 4,6, 0,3, 5,5, 3,0];

    let mut cands = HashSet::<Candidate>::from([(0,0)]);

    for (i, x) in prog.iter().enumerate() {
        dbg!(i, cands.len());
        let mut next = HashSet::<Candidate>::new();

        for c in cands.iter() {
            let ccs = generate_candidates_at(c, 3 * i, *x);
            for cc in ccs {
                next.insert(cc);
            }
        }

        cands = next;
    }


    let lowest = cands.iter().map(|(c, _)| c).min();


    dbg!(lowest);


    // test_num(117440, &base, &prog);



}

pub fn day17() {
    part1();
    part2();
}

