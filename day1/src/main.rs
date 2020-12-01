use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut lines = HashSet::new();
    for line in buf.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        let sub = 2020 - n;
        if lines.contains(&sub) {
            println!("Args: {} {}, answer: {}", n, sub, n * sub);
            return
        }
        lines.insert(n);
    }
    println!("Never found anything :(");
}
