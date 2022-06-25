use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct SegmentDisplay {
    segments_on: [bool; 7],
}

impl SegmentDisplay {
    fn len(&self) -> usize {
        self.segments_on.iter().filter(|s| **s).count()
    }

    fn to_digit(&self) -> u8 {
        match self.segments_on {
            [false, false, false, false, false, false, false] => 0,
            [false, false, true, false, false, true, false] => 1,
            [true, false, true, true, true, false, true] => 2,
            [true, false, true, true, false, true, true] => 3,
            [false, true, true, true, false, true, false] => 4,
            [true, true, false, true, false, true, true] => 5,
            [true, true, false, true, true, true, true] => 6,
            [true, false, true, false, false, true, false] => 7,
            [true, true, true, true, true, true, true] => 8,
            [true, true, true, true, false, true, true] => 9,
            _ => panic!("Unrecognized digit: {:?}", self.segments_on),
        }
    }
}

impl From<&str> for SegmentDisplay {
    fn from(digits: &str) -> Self {
        let mut segments_on = [false; 7];

        for digit in digits.chars() {
            let index = match digit {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                _ => panic!("Unexpected segment encoding: {}", digit),
            };
            segments_on[index] = true;
        }

        SegmentDisplay { segments_on }
    }
}

struct SegmentDisplayTransform {
    segment_mapping: [usize; 7],
}

impl SegmentDisplayTransform {
    fn transform(&self, mut display: SegmentDisplay) -> SegmentDisplay {
        for i in 0..6 {
            display.segments_on[i] = display.segments_on[self.segment_mapping[i]];
        }

        display
    }
}

fn infer_transform(samples: Vec<SegmentDisplay>) -> SegmentDisplayTransform {
    // Number of segments per digit:
    // 0 - 6
    // 1 - 2
    // 2 - 5
    // 3 - 5
    // 4 - 4
    // 5 - 5
    // 6 - 6
    // 7 - 3
    // 8 - 7
    // 9 - 6

    // Digits per number of segments:
    // 2 - 1
    // 3 - 7
    // 4 - 4
    // 5 - 2, 3, 5
    // 6 - 0, 6, 9
    // 7 - 8

    let mut segments_2 = Vec::with_capacity(1);
    let mut segments_3 = Vec::with_capacity(1);
    let mut segments_4 = Vec::with_capacity(1);
    let mut segments_5 = Vec::with_capacity(3);
    let mut segments_6 = Vec::with_capacity(3);
    let mut segments_7 = Vec::with_capacity(1);

    for sample in &samples {
        let segment_list = match sample.len() {
            2 => &mut segments_2,
            3 => &mut segments_3,
            4 => &mut segments_4,
            5 => &mut segments_5,
            6 => &mut segments_6,
            7 => &mut segments_7,
            _ => panic!(
                "There shouldn't be a digit with this number of segments {:?}",
                sample
            ),
        };
        segment_list.push(sample);
    }

    let mut result = SegmentDisplayTransform {
        segment_mapping: [0; 7],
    };

    // TODO Guess which digit it is

    result
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-8-2.txt")?;
    let reader = BufReader::new(file);

    let mut count_part_1 = 0;
    let mut count_part_2 = 0;

    for line in reader.lines().into_iter() {
        let line = line?;
        let mut line_elements = line.split(" ");

        let mut samples = Vec::with_capacity(10);
        let mut output = Vec::with_capacity(4);

        for _ in 0..10 {
            samples.push(SegmentDisplay::from(
                line_elements.next().context("Missing sample")?,
            ));
        }
        if line_elements.next() != Some("|") {
            return Err(anyhow!("Missing \"|\" character"));
        }
        for _ in 0..4 {
            output.push(SegmentDisplay::from(
                line_elements.next().context("Missing output")?,
            ));
        }
        if line_elements.next() != None {
            return Err(anyhow!("Too many elements in input line"));
        }

        for elem in &output {
            match elem.len() {
                // Length of 1, 4, 7 or 8 digits:
                2 | 4 | 3 | 7 => count_part_1 += 1,
                _ => (),
            }
        }

        let transform = infer_transform(samples);

        count_part_2 += output
            .into_iter()
            .map(|d| transform.transform(d))
            .map(|d| d.to_digit())
            .fold(0, |acc, d| acc * 10 + d);
    }

    println!("The answer for part 1 is: {}", count_part_1);
    println!("The answer for part 2 is: {}", count_part_2);

    Ok(())
}
