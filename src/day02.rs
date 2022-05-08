use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() -> Result<()> {
    let file = File::open("input/input-2-2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut aim = 0;

    for line in lines {
        let line = line?;
        let split = line.split(" ").collect::<Vec<&str>>();
        let command = split[0];
        let value = split[1].parse::<i32>().unwrap();

        let (diff_x, diff_y, diff_aim) = match command {
            "forward" => (value, value * aim, 0),
            "up" => (0, 0, -value),
            "down" => (0, 0, value),
            _ => panic!("Unknown command: {}", command),
        };

        pos_x += diff_x;
        pos_y += diff_y;
        aim += diff_aim;
    }

    println!("The result is: {}", pos_x * pos_y);

    Ok(())
}

pub fn main_first() -> Result<()> {
    let file = File::open("input/input-2-2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut pos_x = 0;
    let mut pos_y = 0;

    for line in lines {
        let line = line?;
        let split = line.split(" ").collect::<Vec<&str>>();
        let command = split[0];
        let value = split[1].parse::<i32>().unwrap();

        let (diff_x, diff_y) = match command {
            "forward" => (value, 0),
            "up" => (0, -value),
            "down" => (0, value),
            _ => panic!("Unknown command: {}", command),
        };

        pos_x += diff_x;
        pos_y += diff_y;
    }

    println!("The result is: {}", pos_x * pos_y);

    Ok(())
}
