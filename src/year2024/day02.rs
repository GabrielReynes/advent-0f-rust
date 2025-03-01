use crate::utils::*;

const YEAR: &str = "2024";
const DAY: &str = "2";

pub fn main() {
    let input = get_input(YEAR, DAY).unwrap();

    let res: i32 = input
        .lines()
        .map(|line| line.split(" ").map(|val| val.parse::<i32>().unwrap()))
        .map(|it| it.collect::<Vec<_>>())
        .map(|vec| (is_safe(vec.iter().cloned()) || is_safe_level_2(vec)) as i32)
        .sum();

    submit(YEAR, DAY, "2", &res.to_string());
}

fn is_safe(data: impl Iterator<Item = i32>) -> bool {
    let mut last_diff = 0;
    let mut last_val = 0;
    for (ix, val) in data.enumerate() {
        let diff = val - last_val;
        let diff_abs = diff.abs();
        if ((ix > 0 && (diff_abs < 1 || diff_abs > 3)) || (ix > 1 && (diff ^ last_diff) < 0)) {
            return false;
        }
        last_diff = diff;
        last_val = val;
    }

    return true;
}

fn is_safe_level_2(vec: Vec<i32>) -> bool {
    vec.iter().enumerate().any(|(ix, _)| {
        is_safe(
            vec.iter()
                .enumerate()
                .filter(|(yx, _)| *yx != ix)
                .map(|(_, val)| *val),
        )
    })
}
