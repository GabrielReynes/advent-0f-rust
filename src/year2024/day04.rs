use crate::utils::{get_input, submit};

const DAY: &str = "4";
const YEAR: &str = "2024";
const XMAS: &str = "XMAS";

struct Multizip<T: Iterator>(Vec<T>);

impl<T: Iterator> Iterator for Multizip<T> {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self
            .0
            .iter_mut()
            .filter_map(Iterator::next)
            .collect::<Vec<_>>();
        if value.is_empty() { None } else { Some(value) }
    }
}

fn xmas_count(it: impl DoubleEndedIterator<Item = char> + Clone) -> usize {
    return it.clone().collect::<String>().matches(XMAS).count()
        + it.rev().collect::<String>().matches(XMAS).count();
}

fn multizip_xmas_count(v: Vec<impl Iterator<Item = char>>) -> usize {
    Multizip(v)
        .map(|v| xmas_count(v.into_iter()))
        .sum::<usize>()
}

#[allow(unused)]
pub fn main() {
    let input = get_input(YEAR, DAY).unwrap();

    let line_count = xmas_count(input.chars());

    let column_count = multizip_xmas_count(input.lines().map(str::chars).collect());

    let top_right_count = multizip_xmas_count(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| line[i..].chars())
            .collect(),
    );

    let bottom_left_count = multizip_xmas_count(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| line[..i].chars().rev())
            .collect(),
    );

    let bottom_right_count = multizip_xmas_count(
        input
            .lines()
            .rev()
            .enumerate()
            .map(|(i, line)| line[i..].chars())
            .collect(),
    );

    let top_left_count = multizip_xmas_count(
        input
            .lines()
            .rev()
            .enumerate()
            .map(|(i, line)| line[..i].chars().rev())
            .collect(),
    );

    let res = line_count
        + column_count
        + bottom_left_count
        + bottom_right_count
        + top_left_count
        + top_right_count;

    submit(YEAR, DAY, "1", &res.to_string());
}
