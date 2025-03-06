use regex::Regex;

use crate::utils::{get_input, submit};
use std::ops::Range;

const DAY: &str = "3";
const YEAR: &str = "2024";

#[derive(Debug)]
enum Instr {
    Do(usize),
    Dont(usize),
}

#[derive(Debug)]
struct InstrRanges {
    last: Instr,
    ranges: Vec<Range<usize>>,
}

pub fn main() {
    let input = get_input(YEAR, DAY).unwrap();

    let mul_reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // LEVEL 1

    // let res = mul_reg
    //     .captures_iter(&input)
    //     .map(|capture| capture.extract::<2>())
    //     .map(|(_, [left, right])| left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap())
    //     .sum::<i32>();

    // submit(YEAR, DAY, "1", &res.to_string());

    //LEVEL 2

    let do_reg = Regex::new(r"do(n't)?\(\)").unwrap();

    let InstrRanges { last, mut ranges } = do_reg
        .captures_iter(&input)
        .map(|capture| {
            let mat = capture.get(0).unwrap();
            match capture.get(1) {
                Some(_) => Instr::Dont(mat.start()),
                None => Instr::Do(mat.end()),
            }
        })
        .fold(
            InstrRanges {
                last: Instr::Do(0),
                ranges: Vec::new(),
            },
            |mut acc, e| match acc.last {
                Instr::Do(start) => match e {
                    Instr::Do(_) => acc,
                    Instr::Dont(end) => {
                        acc.ranges.push(start..end);
                        InstrRanges {
                            last: e,
                            ranges: acc.ranges,
                        }
                    }
                },
                Instr::Dont(_) => match e {
                    Instr::Do(_) => InstrRanges {
                        last: e,
                        ranges: acc.ranges,
                    },
                    Instr::Dont(_) => acc,
                },
            },
        );

    if let Instr::Do(start) = last {
        ranges.push(start..input.len());
    }

    let res = mul_reg
        .captures_iter(&input)
        .filter(|capture| {
            ranges
                .iter()
                .any(|range| range.contains(&capture.get(0).unwrap().start()))
        })
        .map(|capture| capture.extract::<2>())
        .map(|(_, [left, right])| left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap())
        .sum::<i32>();

    submit(YEAR, DAY, "2", &res.to_string());
}
