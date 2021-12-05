use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input/input-1-2.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut lines = reader.lines().into_iter();

    let prev_reading_0 = lines.next().unwrap()?.parse::<i32>().unwrap();
    let mut prev_reading_1 = lines.next().unwrap()?.parse::<i32>().unwrap();
    let mut prev_reading_2 = lines.next().unwrap()?.parse::<i32>().unwrap();
    let mut prev_sum = prev_reading_0 + prev_reading_1 + prev_reading_2;

    for line in lines {
        let next_reading = line?.parse::<i32>().unwrap();
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

fn main_first() -> io::Result<()> {
    let file = File::open("input/input-1-2.txt")?;
    let reader = BufReader::new(file);

    let mut result = 0;
    let mut lines = reader.lines().into_iter();
    let mut prev_reading = lines.next().unwrap()?.parse::<i32>().unwrap();

    for line in lines {
        let next_reading = line?.parse::<i32>().unwrap();
        if next_reading > prev_reading {
            result += 1;
        }
        prev_reading = next_reading;
    }

    println!("The result is: {}", result);

    Ok(())
}
