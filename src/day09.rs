use anyhow::{Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const MAX_POINT_VALUE: u8 = 9;

#[derive(Debug)]
struct Heightmap {
    height: usize,
    width: usize,
    values: Vec<u8>,
}

impl Heightmap {
    fn from_input(input: Vec<String>) -> Heightmap {
        let height = input.len();
        let width = input.first().map(|l| l.len()).unwrap_or(0);
        let mut values = vec![0; height * width];

        for h in 0..height {
            for (w, c) in input[h].char_indices() {
                values[h * width + w] = c.to_digit(10).unwrap() as u8;
            }
        }

        Heightmap {
            width,
            height,
            values,
        }
    }

    #[inline]
    fn point_at(&self, x: isize, y: isize) -> u8 {
        if x < 0 || (x as usize) >= self.width || y < 0 || (y as usize) >= self.height {
            MAX_POINT_VALUE
        } else {
            self.values[(y as usize) * self.width + (x as usize)]
        }
    }

    fn is_low_point(&self, x: isize, y: isize) -> bool {
        let point = self.point_at(x, y);

        if point >= self.point_at(x, y - 1)
            || point >= self.point_at(x - 1, y)
            || point >= self.point_at(x + 1, y)
            || point >= self.point_at(x, y + 1)
        {
            false
        } else {
            true
        }
    }
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-9-2.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().into_iter().map(|l| l.unwrap());
    let heightmap = Heightmap::from_input(lines.collect::<Vec<String>>());

    let mut result: u32 = 0;
    for y in (0..heightmap.height).map(|i| i as isize) {
        for x in (0..heightmap.width).map(|i| i as isize) {
            if heightmap.is_low_point(x, y) {
                result += heightmap.point_at(x, y) as u32 + 1;
            }
        }
    }    

    println!("The result is: {}", result);

    Ok(())
}
