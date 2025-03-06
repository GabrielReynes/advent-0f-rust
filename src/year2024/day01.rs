use std::{collections::HashMap, iter::zip};

use regex::Regex;

use crate::utils::*;

pub fn main() {
    let year = "2024";
    let day = "1";

    let input = get_input(year, day).unwrap();

    let reg = Regex::new(r"\s+").unwrap();
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut line_iter = reg.split(line).map(|val| val.parse::<i64>().unwrap());
            (line_iter.next().unwrap(), line_iter.next().unwrap())
        })
        .unzip();


    // LEVEL 1
    /*
    left.sort();
    right.sort();

    let res: i64 = zip(left, right)
        .map(|(left, right)| (left - right).abs())
        .sum();

    submit(year, day, "1", &res.to_string());
    */

    // LEVEL 2
    let mut occ_hash = HashMap::new();

    for val in right {
        occ_hash.insert(val, occ_hash.get(&val).unwrap_or(&0) + 1);
    }

    let res: i64 = left
        .iter()
        .map(|e| e * occ_hash.get(&e).unwrap_or(&0))
        .sum();

    submit(year, day, "2", &res.to_string());
}
