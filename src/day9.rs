use std::fs;

fn read_input(file_path: String) -> String {
    println!("Reading input");

    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

#[derive(Debug, Copy, Clone)]
enum Block {
    Free(u64),
    File(u64, u64)
}


fn part1() {
    // In part 1, because the max block length is 9, we can probably work with expanded strings
    let input = read_input("input.txt".to_string());

    fn expand(s:&String) -> Vec<Block> {
        let mut r = Vec::new();
        let mut id = 0 as u64;
        let mut free_toggle = false;

        for c in s.chars() {
            let len = c.to_digit(10).expect("oops, not a digit") as u64;
            if free_toggle {
                r.push(Block::Free(len));
                free_toggle = !free_toggle;
            } else {
                r.push(Block::File(id, len));
                id += 1;
                free_toggle = !free_toggle;
            }
        }

        r
    }
    

    // Tries to compact a filing system, returning true if it was able to do something
    fn compact(mut fs: Vec<Block>) -> (bool, Vec<Block>) {
        if !fs.is_empty() {
            let &last = fs.last().unwrap();
            match last {
                Block::Free(_) => {
                    fs.pop();
                    (true, fs)
                },
                Block::File(id, len) => {
                    let first_free = fs.iter_mut().enumerate().find(|(_, b)| {
                        match b {
                            Block::Free(len) => *len > 0,
                            _ => false
                        }
                    });
    
                    match first_free {
                        Some((i, Block::Free(free_len))) => {
                            fs[i] = Block::Free(*free_len - 1);
                            if i > 0 {
                                match fs[i - 1] {
                                    Block::File(prev_id, prev_size) if prev_id == id => {
                                        fs[i - 1] = Block::File(id, prev_size + 1);
                                    },
                                    _ => {
                                        fs.insert(i, Block::File(id, 1));
                                    }
                                }
    
                                fs.pop();
                                if len > 1 {
                                    fs.push(Block::File(id, len - 1))
                                };
                            } else {
                                fs.insert(i, Block::File(id, 1));

                                fs.pop();
                                if len > 1 {
                                    fs.push(Block::File(id, len - 1))
                                };
                            }
    
                            (true, fs)
                        }
                        _ => { (false, fs) }
                    }
                }


            }

        } else {
            (false, fs)        
        }

    }

    fn checksum(fs:&Vec<Block>) -> u64 {
        let (_, tot) = fs.into_iter().fold((0 as u64, 0 as u64), |(i, tot), block| {
            match block {
                Block::File(id, len) => {
                    let before = (i as i64) - 1;
                    let end = i + len - 1;
                    let sum = (end * (end + 1) / 2) - (before * (before + 1) / 2) as u64;
                    (i + len, tot + sum * id)
                }
                Block::Free(len) => { (i + len, tot ) }        
            }
        });

        tot

    }

    let mut fs = expand(&input);

    dbg!(&fs);

    while {
        let (cont, ret) = compact(fs);
        fs = ret;
        cont
    } {}

    dbg!(checksum(&fs));





}


fn part2() {
    

}

pub fn day9() {
    part1();
    part2();
}

