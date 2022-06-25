use anyhow::{anyhow, Context, Result};
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct SegmentDisplay {
    segments_on: [bool; 7],
}

impl SegmentDisplay {
    fn from_positions(positions: Vec<usize>) -> SegmentDisplay {
        let mut segments_on = [false; 7];

        for position in positions {
            segments_on[position] = true;
        }

        SegmentDisplay { segments_on }
    }

    fn from_position_negative(positions: Vec<usize>) -> SegmentDisplay {
        let mut segments_on = [true; 7];

        for position in positions {
            segments_on[position] = false;
        }

        SegmentDisplay { segments_on }
    }

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

    fn find_different_segment(&self, other: &SegmentDisplay) -> Option<usize> {
        self.segments_on
            .iter()
            .zip(other.segments_on.iter())
            .position(|(a, b)| a != b)
    }

    fn remaining_segment(&self, positions: Vec<usize>) -> Option<usize> {
        self.segments_on
            .iter()
            .enumerate()
            .find(|r| !positions.contains(&r.0) && *r.1)
            .iter()
            .map(|r| r.0)
            .last()
    }

    fn common_segments(&self, other: &SegmentDisplay) -> Vec<usize> {
        let mut result = Vec::with_capacity(7);

        for i in 0..6 {
            if self.segments_on[i] == other.segments_on[i] {
                result.push(i);
            }
        }

        result
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

#[derive(Debug)]
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
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg

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

    let digit_1 = *segments_2.first().unwrap();
    let digit_4 = *segments_4.first().unwrap();
    let digit_7 = *segments_3.first().unwrap();

    // 7 - 1 = a
    let mapping_a = digit_7.find_different_segment(digit_1).unwrap();

    // [segments_6] - 7 = c
    let mapping_c = find_different_segment_in_vec(&segments_6, digit_7).unwrap();

    // 1 - c = f
    let mapping_f = digit_1.remaining_segment(vec![mapping_c]).unwrap();

    // [segments_5] & 4 = d
    let mapping_d = find_common_segment_in_vec(&segments_5, digit_4).unwrap();

    // 4 - c - d - f = b
    let mapping_b = digit_4
        .remaining_segment(vec![mapping_c, mapping_d, mapping_f])
        .unwrap();

    let digit_all_minus_e_g = SegmentDisplay::from_position_negative(vec![
        mapping_a, mapping_b, mapping_c, mapping_d, mapping_f,
    ]);

    // eg & [segments_5] = g
    let mapping_g = find_common_segment_in_vec(&segments_5, &digit_all_minus_e_g).unwrap();

    // eg - g = e
    let mapping_e = digit_all_minus_e_g.remaining_segment(vec![mapping_g]).unwrap();

    // First line - correct: 3 4 0 5 6 1 2
    //                       3 4 2 2 0 1 5
    //                       a b c d e f g

    SegmentDisplayTransform {
        segment_mapping: [
            mapping_a, mapping_b, mapping_c, mapping_d, mapping_e, mapping_f, mapping_g,
        ],
    }
}

fn find_different_segment_in_vec(
    segments: &Vec<&SegmentDisplay>,
    from: &SegmentDisplay,
) -> Option<usize> {
    for segment in segments {
        match (*segment).find_different_segment(from) {
            Some(result) => return Some(result),
            _ => (),
        }
    }

    None
}

fn find_common_segment_in_vec(
    segments: &Vec<&SegmentDisplay>,
    from: &SegmentDisplay,
) -> Option<usize> {
    let mut current_common = segments[0].common_segments(from);

    for window in segments.windows(2) {
        current_common = window[0]
            .common_segments(window[1])
            .into_iter()
            .filter(|c| current_common.contains(c))
            .collect();
    }

    if current_common.len() == 1 {
        Some(current_common[0])
    } else {
        None
    }
}

pub fn main() -> Result<()> {
    let file = File::open("input/input-8-1.txt")?;
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
        println!("Transform: {:?}", transform);

        count_part_2 += output
            .into_iter()
            .inspect(|_| println!("---"))
            .inspect(|d| println!("Output: {:?}", d))
            .map(|d| transform.transform(d))
            .inspect(|d| println!("Transformed: {:?}", d))
            .map(|d| d.to_digit())
            .fold(0, |acc, d| acc * 10 + d);
    }

    println!("The answer for part 1 is: {}", count_part_1);
    println!("The answer for part 2 is: {}", count_part_2);

    Ok(())
}
