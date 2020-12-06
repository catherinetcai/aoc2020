use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut datas = vec![HashSet::new()];

    for line in buf.lines() {
        let unwrapped = line.unwrap();
        println!("Line: {}", unwrapped);

        if unwrapped.trim().is_empty() {
            // We know that this is the end of a passport builder
            println!("This is an empty line!");
            datas.push(HashSet::new());
        } else {
            for k in unwrapped.split("") {
                if k.is_empty() {
                    continue
                }
                datas.last_mut().unwrap().insert(k.to_string());
            }
        }
    }

    let mut sum = 0;

    for data in datas {
        let length = data.len();
        println!("Data: {:?}, Length: {}", data, length);
        sum += length;
    }

    println!("Sum: {}", sum);
}
