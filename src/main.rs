extern crate nix;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("/proc/mounts").unwrap();
    let reader = BufReader::new(&file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let fields: Vec<&str> = line.split_whitespace().collect();
                println!("{} {}", fields[0], fields[1])
            },
            Err(_) => panic!(),
        }
    }
}
