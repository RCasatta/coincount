extern crate data_encoding;
extern crate plotlib;

use std::io;
use data_encoding::HEXLOWER;
use std::collections::HashSet;
use std::iter::Iterator;
use std::sync::mpsc::sync_channel;
use std::thread;
use plotlib::view::ContinuousView;
use plotlib::style::Line;

#[derive(Debug)]
struct InputLine {
    input: bool,
    height: u32,
    key: Vec<u8>
}

struct Counter {
    set: HashSet<Vec<u8>>,
    last_spent: u32,
    spent: u32,
    size: u32,
    list: Vec<f64>,
}

impl Counter {

    fn new(size : u32) -> Counter {
        Counter {
            set: HashSet::new(),
            last_spent: 0,
            spent: 0,
            size,
            list: vec![],
        }
    }

    pub fn count(&mut self, line: &InputLine) {
        if line.height % self.size == 0 {
            if self.set.len() > 0 {
                let i = self.spent - self.last_spent;
                if i > 0 {
                    self.list.push(i as f64 / self.set.len() as f64);
                }
            }

            self.set = HashSet::new();
            self.last_spent = self.spent;
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

    fn save_graph(&self) {
        if self.list.len()>0 {
            let mut reduced = self.list.clone();
            let mut n_reduce = 0f64;
            while reduced.len()>500 {
                reduced=reduced
                    .chunks(2)
                    .map(|el| (el[0] + el.get(1).unwrap_or(&el[0]))*0.5 )
                    .collect();
                n_reduce = n_reduce + 1.0;
            }
            let mut elements = Vec::new();
            let size = self.size as f64;
            let mut block = size.clone();
            for el in &reduced {
                elements.push((block.clone(),el.clone()));
                block = block +  size * n_reduce;
            }
            //let elements = self.list.enumerate().map(|el| (el.0 as f64, el.1)).collect();
            let l1 = plotlib::line::Line::new(&elements[..]).style( plotlib::line::Style::new().colour("burlywood"));
            let v = ContinuousView::new().add(&l1);
            plotlib::page::Page::single(&v).save(format!("{}.svg", self.size));
        }

    }
}

fn main() {
    let sizes = [2u32,4,16,64,144,256,1024,4096,16384,65536];
    let (sender, receiver) = sync_channel(1000);

    let mut counters : Vec<Counter> = Vec::new();
    for size in sizes.iter() {
        counters.push(Counter::new(size.clone()));
    }
    let mut total = 0u32;

    let t = thread::spawn(move || {
        loop {
            match receiver.recv().unwrap() {
                Some(line) => {
                    for current_counter in counters.iter_mut() {
                        current_counter.count(&line);
                    }
                    if !line.input {
                        total = total + 1;
                    }
                },
                None => {
                    println!("Fin");
                    break;
                }
            };
        }
        println!("Total outputs {}", total);
        for current_counter in counters {
            current_counter.print(total);
            current_counter.save_graph();
        }
    });

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
                sender.send(Some(line)).unwrap();
            }
            Err(error) => panic!("error: {}", error),
        }
    }
    sender.send(None).unwrap();
    t.join().unwrap();
}


fn parse(line_str : Vec<&str>) -> InputLine {
    let input = "i".eq(line_str[0]);
    let height = line_str[1].parse::<u32>().unwrap();
    let mut key = HEXLOWER.decode(line_str[2].as_bytes()).unwrap();
    let num = transform_u32_to_array_of_u8(line_str[3].parse::<u32>().unwrap() );
    if input {
        key.reverse();  //bitcoin-iterate serve the tx hash in big endian while tx input in little endian
    }
    key.extend(num.to_vec());

    InputLine {
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