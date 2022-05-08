use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_DISTANCE: usize = 2000;

pub fn main() -> Result<()> {
    println!("TODO");
    Ok(())
}

pub fn main_first() -> Result<()> {
    let file = File::open("input/input-7-2.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines().into_iter().map(|l| l.unwrap());

    // O(n) solution for the first part:
    // - a[i] - numbers of crabs at position i
    // - s[i] - sum of a[i] until i value
    // - t[i] - sum of i * a[i] until i value
    // - f[i] = i * (2 * s[i] - s[MAX]) + t[MAX] - 2 * t[i]
    // ^^^ the solution I've got by solving F(x) as sum in form (x - target position):
    // F(x) = Sum(i = 0..MAX) a[i] * |i - x|
    // |i - x| - distance (absolute value) between crab and target
    // The minimal value of F(0..MAX) is the solution for the problem.

    let a = &mut [0 as i32; MAX_DISTANCE];
    let s = &mut [0 as i32; MAX_DISTANCE];
    let t = &mut [0 as i32; MAX_DISTANCE];
    let f = &mut [0 as i32; MAX_DISTANCE];

    let first_line = lines.next().context("Missing first line")?;
    let values = first_line
        .split(",")
        .map(|s| s.parse::<i32>().context("Couldn't parse number"));

    for value in values {
        let value = value?;
        a[value as usize] += 1;
    }

    s[0] = a[0];

    for i in 1..MAX_DISTANCE {
        s[i] = a[i] + s[i - 1];
        t[i] = (i as i32) * a[i] + t[i - 1];
    }

    for i in 0..MAX_DISTANCE {
        f[i] = (i as i32) * (2 * s[i] - s[MAX_DISTANCE - 1]) + t[MAX_DISTANCE - 1] - 2 * t[i];
    }

    let minimum_fuel = f
        .iter()
        .min()
        .context("Couldn't calculate minimum from empty iterator")?;

    println!("The result is: {}", minimum_fuel);

    Ok(())
}
