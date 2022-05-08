use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const DAYS: usize = 256;
type Memory = [[u64; 9]; DAYS + 1];

// Part 1.
#[allow(dead_code)]
fn calculate_population_naive(state: u64, days_left: u64) -> u64 {
    if days_left == 0 {
        1
    } else if state == 0 {
        calculate_population_naive(6, days_left - 1) + calculate_population_naive(8, days_left - 1)
    } else {
        calculate_population_naive(state - 1, days_left - 1)
    }
}

// Part 2.
fn calculate_population_memoize(memory: &mut Memory, state: u64, days_left: u64) -> u64 {
    let mut memoized_value = memory[days_left as usize][state as usize];

    if memoized_value == 0 {
        memoized_value = match (state, days_left) {
            (_, 0) => 1,
            (0, _) => {
                calculate_population_memoize(memory, 6, days_left - 1)
                    + calculate_population_memoize(memory, 8, days_left - 1)
            }
            (_, _) => calculate_population_memoize(memory, state - 1, days_left - 1),
        };
        memory[days_left as usize][state as usize] = memoized_value;
    }

    memoized_value
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-6-2.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().into_iter().map(|l| l.unwrap());

    let memory = &mut [[0; 9]; DAYS + 1];

    let initial_states: u64 = lines
        .next()
        .context("Missing first line")?
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        // .map(|s| calculate_naive(s, DAYS as u64))
        .map(|s| calculate_population_memoize(memory, s, DAYS as u64))
        .sum();

    println!("The result is: {}", initial_states);

    Ok(())
}
