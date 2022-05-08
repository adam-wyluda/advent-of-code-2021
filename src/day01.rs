use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<()> {
    let file = File::open("input/input-1-2.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut lines = reader.lines().into_iter();

    let prev_reading_0 = lines
        .next()
        .context("Missing first reading")??
        .parse::<i32>()?;
    let mut prev_reading_1 = lines
        .next()
        .context("Missing second reading")??
        .parse::<i32>()?;
    let mut prev_reading_2 = lines
        .next()
        .context("Missing third reading")??
        .parse::<i32>()?;
    let mut prev_sum = prev_reading_0 + prev_reading_1 + prev_reading_2;

    for line in lines {
        let next_reading = line?.parse::<i32>()?;
        let next_sum = next_reading + prev_reading_2 + prev_reading_1;

        if next_sum > prev_sum {
            result += 1;
        }

        prev_reading_1 = prev_reading_2;
        prev_reading_2 = next_reading;
        prev_sum = next_sum;
    }

    println!("The result is: {}", result);

    Ok(())
}

pub fn main_first() -> Result<()> {
    let file = File::open("input/input-1-2.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut lines = reader.lines().into_iter();
    let mut prev_reading = lines
        .next()
        .context("Missing first reading")??
        .parse::<i32>()?;

    for line in lines {
        let next_reading = line?.parse::<i32>()?;
        if next_reading > prev_reading {
            result += 1;
        }
        prev_reading = next_reading;
    }

    println!("The result is: {}", result);

    Ok(())
}
