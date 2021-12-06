use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const DAYS: usize = 256;
type Memory = [[u64; 9]; DAYS + 1];

// Part 1.
fn calculate_naive(s: u64, n: u64) -> u64 {
    if n == 0 {
        1
    } else if s == 0 {
        calculate_naive(6, n - 1) + calculate_naive(8, n - 1)
    } else {
        calculate_naive(s - 1, n - 1)
    }
}

// Part 2.
fn calculate_memoize(memory: &mut Memory, s: u64, n: u64) -> u64 {
    if memory[n as usize][s as usize] == 0 {
        memory[n as usize][s as usize] = match (s, n) {
            (_, 0) => 1,
            (0, _) => calculate_memoize(memory, 6, n - 1) + calculate_memoize(memory, 8, n - 1),
            (_, _) => calculate_memoize(memory, s - 1, n - 1),
        };
    }

    memory[n as usize][s as usize]
}

fn main() -> io::Result<()> {
    let file = File::open("input/input-6-2.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().into_iter().map(|l| l.unwrap());

    let initial_states: u64 = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        // .map(|s| calculate_naive(s, DAYS as u32))
        .map(|s| calculate_memoize(&mut [[0; 9]; DAYS + 1], s, DAYS as u64))
        .sum();

    println!("The result is: {}", initial_states);

    Ok(())
}
