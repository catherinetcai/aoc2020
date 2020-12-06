use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    // We're doing 0-127
    let max_row = 127;
    // We're doing 0-7
    let max_col = 7;
    let mut max_seat = 0;

    for line in buf.lines() {
        // Indices 0-6 are the row
        // Indices 7-9 are the col
        let mut first = line.unwrap().clone();
        let second = first.split_off(7);

        println!("First: {}, second: {}", first.chars().rev().collect::<String>(),
            second.chars().rev().collect::<String>());

        let row_num = find_row(first, 0, max_row);
        let col_num = find_col(second, 0, max_col);

        println!("Row: {}, col: {}", row_num, col_num);
        let seat_num = find_seat(row_num, col_num);

        max_seat = max(max_seat, seat_num);
    }

    println!("Max seat: {}", max_seat);
}

fn find_row(mut input: String, low: u32, high: u32) -> u32 {
    let mid = low + (high - low)/2;
    println!("Row mid: {}, for low: {}, high: {}, input: {}", mid, low, high, input);
    match input.pop() {
        Some(x) => {
            if x == 'F' {
                return find_row(input, low, mid);
            }
            return find_row(input, mid, high);
        },
        None => {
            return mid;
        }
    }
}

fn find_col(mut input: String, low: u32, high: u32) -> u32 {
    let mid = low + (high - low)/2;
    println!("Col mid: {}, for low: {}, high: {}, input: {}", mid, low, high, input);
    match input.pop() {
        Some(x) => {
            if x == 'L' {
                return find_col(input, low, mid);
            }
            return find_col(input, mid+1, high);
        },
        None => {
            return mid;
        }
    }
}

fn find_seat(row: u32, col: u32) -> u32{
    (row * 8) + col
}
