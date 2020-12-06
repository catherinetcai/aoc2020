use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut datas = vec![HashSet::new()];
    let mut datas_two: Vec<HashMap<String, u32>> = vec![HashMap::new()];

    // Solution 1
    for line in buf.lines() {
        let unwrapped = line.unwrap();
        println!("Line: {}", unwrapped);

        if unwrapped.trim().is_empty() {
            // We know that this is the end of a passport builder
            println!("This is an empty line!");
            datas.push(HashSet::new());
            datas_two.push(HashMap::new());
        } else {
            let last = datas_two.last_mut().unwrap();
            *last.entry("count".to_string()).or_insert(0) += 1;
            for k in unwrapped.split("") {
                if k.is_empty() {
                    continue
                }
                datas.last_mut().unwrap().insert(k.to_string());
                *last.entry(k.to_string()).or_insert(0) += 1;
            }
        }
    }

    let mut sum_one = 0;

    for data in datas {
        let length = data.len();
        sum_one += length;
    }

    println!("Sum: {}", sum_one);

    let mut sum_two = 0;

    for data in datas_two {
        println!("Data two: {:?}", data);
        for (k, v) in data.iter() {
            if k == "count" {
                continue
            }
            if *v == data["count"] {
                sum_two += 1;
            }
        }
    }

    println!("Sum two: {}", sum_two);
}
