extern crate data_encoding;

use std::io;
use data_encoding::HEXLOWER;
use std::collections::HashSet;

#[derive(Debug)]
struct Line {
    input: bool,
    height: u32,
    key: Vec<u8>
}

struct Counter {
    set: HashSet<Vec<u8>>,
    spent: u32,
    size: u32,
}

impl Counter {

    fn new(size : u32) -> Counter {
        Counter {
            set: HashSet::new(),
            spent: 0,
            size,
        }
    }

    pub fn count(&mut self, line: &Line) {
        if line.height % self.size == 0 {
            self.set = HashSet::new();
        }

        if line.input {
            if self.set.contains(&line.key) {
                self.spent = self.spent + 1;
            }
        } else {
            self.set.insert(line.key.clone());
        }
    }

    fn print(&self, total : u32) {
        println!("size: {} spent: {} ratio:{}", self.size, self.spent, self.spent as f64 / total as f64);
    }
}

fn main() {
    let sizes = [2u32,4,16,64,144,256,1024,4096,16384];
    let mut counters : Vec<Counter> = Vec::new();
    for size in sizes.iter() {
        counters.push(Counter::new(size.clone()));
    }
    let mut total = 0u32;
    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //println!("{}", buffer);
                let line : Vec<&str> = buffer.split_whitespace().collect();
                let line = parse(line);

                for current_counter in counters.iter_mut() {
                    current_counter.count(&line);
                }
                if !line.input {
                    total = total + 1;
                }
            }
            Err(error) => panic!("error: {}", error),
        }
    }

    println!("Total outputs {}", total);
    for current_counter in counters {
        current_counter.print(total);
    }
}

fn parse(line_str : Vec<&str>) -> Line {
    let input = "i".eq(line_str[0]);
    let height = line_str[1].parse::<u32>().unwrap();
    let mut key = HEXLOWER.decode(line_str[2].as_bytes()).unwrap();
    let num = transform_u32_to_array_of_u8(line_str[3].parse::<u32>().unwrap() );
    if input {
        key.reverse();  //bitcoin-iterate serve the tx hash in big endian while tx input in little endian
    }
    key.extend(num.to_vec());

    Line {
        input,
        height,
        key,
    }
}

fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}