use anyhow::Result;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::str;

const U8_ASCII_1: u8 = '1' as u8;
const U8_ASCII_0: u8 = '0' as u8;

pub fn main() -> Result<()> {
    let file = File::open("input/input-3-2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|res| res.unwrap())
        .collect::<Vec<String>>();

    // Reuse two buffers to avoid allocations in find_rating()
    let mut buffer_a = Vec::with_capacity(lines.len());
    let mut buffer_b = Vec::with_capacity(lines.len());

    let oxygen_rating = find_rating(&lines, &mut buffer_a, &mut buffer_b, true);
    let co2_rating = find_rating(&lines, &mut buffer_a, &mut buffer_b, false);

    let oxygen_rating = isize::from_str_radix(str::from_utf8(oxygen_rating).unwrap(), 2).unwrap();
    let co2_rating = isize::from_str_radix(str::from_utf8(co2_rating).unwrap(), 2).unwrap();

    println!("The result is: {}", oxygen_rating * co2_rating);

    Ok(())
}

fn find_rating<'a>(
    lines: &'a Vec<String>,
    buffer_a: &mut Vec<&'a [u8]>,
    buffer_b: &mut Vec<&'a [u8]>,
    seek_most_common: bool,
) -> &'a [u8] {
    buffer_a.clear();
    buffer_b.clear();

    let ratings = buffer_a;
    let output = buffer_b;

    ratings.extend(lines.iter().map(|l| l.as_bytes()));

    let width = lines[0].len();
    for bit_position in 0..width {
        limit_ratings(&ratings, output, bit_position, seek_most_common);

        if output.len() == 1 {
            break;
        }

        if bit_position < width - 1 {
            std::mem::swap(ratings, output);
            output.clear();
        }
    }

    assert_eq!(output.len(), 1, "There should only be a single result");
    output[0]
}

fn limit_ratings<'a>(
    ratings: &Vec<&'a [u8]>,
    output: &mut Vec<&'a [u8]>,
    bit_position: usize,
    seek_most_common: bool,
) {
    assert!(output.is_empty(), "Output must be a clean vector");

    let mut count_of_1 = 0;

    for rating in ratings {
        count_of_1 += match rating[bit_position] {
            U8_ASCII_1 => 1,
            U8_ASCII_0 => 0,
            c => panic!("Unknown character: {}", c),
        };
    }

    // Split odd/even cases, as integer division doesn't round up
    let most_common = if (ratings.len() % 2 == 0 && count_of_1 >= ratings.len() / 2)
        || (ratings.len() % 2 == 1 && count_of_1 > ratings.len() / 2)
    {
        U8_ASCII_1
    } else {
        U8_ASCII_0
    };

    for rating in ratings {
        let is_same_as_most_common = rating[bit_position] == most_common;
        if is_same_as_most_common == seek_most_common {
            output.push(rating);
        }
    }
}

// -------------------------------------------------------

pub fn main_first() -> Result<()> {
    let file = File::open("input/input-3-2.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().into_iter();

    let mut total_count = 0;
    let mut frequency = Vec::new();

    for line in lines {
        let line = line?;

        // Initialize frequency vector for the first line
        if frequency.is_empty() {
            for _ in 0..line.len() {
                frequency.push(0);
            }
        }
        total_count += 1;

        for (i, c) in line.chars().enumerate() {
            frequency[i] += match c {
                '1' => 1,
                '0' => 0,
                _ => panic!("Unknown character: {}", c),
            }
        }
    }

    let mut power = 1;
    let mut gamma_value = 0;
    let mut epsilon_value = 0;

    for i in 0..frequency.len() {
        if frequency[frequency.len() - 1 - i] > total_count / 2 {
            gamma_value += power;
        } else {
            epsilon_value += power;
        }

        power *= 2;
    }

    println!("The result is: {}", gamma_value * epsilon_value);

    Ok(())
}
