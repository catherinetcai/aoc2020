use std::io::{BufReader};
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let mut grid = Vec::new();

    let mut line_length : usize = 0;
    for (i, line) in buf.lines().enumerate() {
        let unwrapped_line = line.unwrap();
        if i == 0 {
            line_length = unwrapped_line.chars().count();
        }
        let mut row = Vec::new();

        for c in unwrapped_line.chars() {
            row.push(c);
        }

        grid.push(row);
    }

    println!("Line length: {}", line_length);
    // HACK: This is super ugly, but you can only run these one at a time until I find a
    // better way for cloning these grids. Otherwise, this alters the grid *in place* which
    // screws it up for the next round!
    println!("Tree count for step 1: {}", counter(&mut grid, line_length, 1, 1));
    // println!("Tree count for step 3: {}", counter(&mut grid(), line_length, 1, 3));
    // println!("Tree count for step 5: {}", counter(&mut grid(), line_length, 1, 5));
    // println!("Tree count for step 7: {}", counter(&mut grid(), line_length, 1, 7));
    // println!("Tree count for step 2: {}", counter(&mut grid(), line_length, 2, 1));

}

fn counter(grid: &mut Vec<Vec<char>>, line_length: usize, dstep: usize, rstep: usize) -> u32 {
    let mut tree_counter : u32 = 0;
    let mut step_tracker = 0;

    // We just need to know when to wrap
    'outer: for i in (0..grid.len()).step_by(dstep) {
        // println!("{:?}", grid[i]);
        // We're in the last row, so we can stop
        if i == grid.len() - dstep - 1{
            break
        }
        for j in (0..grid[i].len()).step_by(rstep) {
            let next_row = i+dstep;

            let mut next_steps = step_tracker + rstep as usize;
            // println!("Next steps before adjust: {}", next_steps);

            if next_steps >= line_length {
                next_steps = next_steps - line_length;
            }

            step_tracker = next_steps;

            let next_move = grid[next_row][next_steps];
            if next_move == '#' {
                grid[next_row][next_steps] = 'X';
                tree_counter += 1;
            } else {
                grid[next_row][next_steps] = 'O';
            }
            continue 'outer;
        }
    }

    println!("Fell off the map, tree count: {}", tree_counter);

    return tree_counter;
}
