use anyhow::{anyhow, Context, Error, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<()> {
    let file = File::open("input/input-8-2.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines().into_iter() {
        let line = line?;
        let mut line_elements = line.split(" ");

        let mut samples = Vec::with_capacity(10);
        let mut output = Vec::with_capacity(4);

        for _ in 0..10 {
            samples.push(line_elements.next().context("Missing sample")?);
        }
        if line_elements.next() != Some("|") {
            return Err(anyhow!("Missing \"|\" character"));
        }
        for _ in 0..4 {
            output.push(line_elements.next().context("Missing output")?);
        }
        if line_elements.next() != None {
            return Err(anyhow!("Too many elements in input line"));
        }

        for elem in &output {
            match elem.len() {
                // Length of 1, 4, 7 or 8 digits:
                2 | 4 | 3 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("The answer is: {}", count);

    Ok(())
}
