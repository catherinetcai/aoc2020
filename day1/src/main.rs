use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut lines = HashSet::new();
    let mut all_nums: Vec<i32> = Vec::new();

    // Solution 1
    for line in buf.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        let sub = 2020 - n;
        if lines.contains(&sub) {
            println!("Args: {} {}, answer: {}", n, sub, n * sub);
        }
        lines.insert(n);
        all_nums.push(n)
    }
    println!("Never found anything :(");

    // Solution 2 - Three sum
    all_nums.sort();

    for i in 0..all_nums.len()-2 {
        let mut l = i + 1;
        let mut r = all_nums.len()-1;

        while (l < r) {
            if all_nums[i] + all_nums[l] + all_nums[r] == 2020 {
                let product = all_nums[i] * all_nums[l] * all_nums[r];
                println!("THREE SUM: {}", product);
                return
            } else if all_nums[i] + all_nums[l] + all_nums[r] < 2020 {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }
}
