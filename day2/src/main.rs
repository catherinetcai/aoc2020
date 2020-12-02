use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let mut counter: i32 = 0;
    // Format of a line is
    // min-max <letter>: <password>

    let re = Regex::new(r"(?P<min>\d+)\-(?P<max>\d+)\s*(?P<letter>[a-zA-Z]{1}):\s*(?P<password>[a-zA-Z]+)").unwrap();
    for line in buf.lines() {
        let n = line.unwrap();
        if valid_password(&re, n.as_str()) {
            counter += 1;
        } else {
            println!("Didn't find a valid password: {}", n);
        }
    }
    println!("Final count of valid passwords: {}", counter);
}

fn valid_password(re: &Regex, input: &str) -> bool {
    // let matched = re.captures("1-8 n: dpwpmhknmnlglhjtrbpx");
    match re.captures(input) {
        Some(matched) => {
            let min: usize = matched.name("min").unwrap().as_str().parse().unwrap();
            let max: usize = matched.name("max").unwrap().as_str().parse().unwrap();
            let letter = matched.name("letter").unwrap().as_str().to_string();
            let password = matched.name("password").unwrap().as_str();

            let mut matches = 0;
            for c in password.chars() {
                println!("Testing char: {}, letter: {}", c.to_string(), letter.to_string());
                if c.to_string() == letter.to_string() {
                    matches += 1
                }
            }

            println!("Parsed string: min {}, max {}, letter {}, password {}, count: {}", min, max, letter, password, matches);
            if matches >= min && matches <= max {
                return true;
            }
            return false;
        },
        None => {
            println!("Issue with input: {}", input);
            return false;
        }
    }
}
