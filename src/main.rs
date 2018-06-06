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

fn main() {
    let mut set = HashSet::new();
    let mut n_input = 0u32;
    let mut n_output = 0u32;
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
                //println!("{:?}", line);
                if line.input {
                    set.remove(&line.key);
                    n_input = n_input +1;
                } else {
                    set.insert(line.key.clone());
                    n_output = n_output +1;
                }

            }
            Err(error) => panic!("error: {}", error),
        }
    }
    println!("set size: {}" , set.len());
    println!("n_input: {}" , n_input);
    println!("n_output: {}" , n_output);
}

fn parse(line_str : Vec<&str>) -> Line {
    let input = "i".eq(line_str[0]);
    let height = line_str[1].parse::<u32>().unwrap();
    let mut key = HEXLOWER.decode(line_str[2].as_bytes()).unwrap();
    let num = transform_u32_to_array_of_u8(line_str[3].parse::<u32>().unwrap() );
    if input {
        key.reverse()
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