use crate::utils::thats_weird;

fn solve_day01_1(entries: Vec<u64>) -> Option<u64> {
    for (i1, x) in entries.iter().enumerate() {
        for (i2, y) in entries.iter().enumerate() {
            if i1 == i2 {
                continue
            }
            if x + y == 2020 {
                return Some(x * y)
            }
        }
    }
    None
}

fn solve_day01_2(entries: Vec<u64>) -> Option<u64> {
    for (i1, x) in entries.iter().enumerate() {
        for (i2, y) in entries.iter().enumerate() {
            for (i3, z) in entries.iter().enumerate() {
                if i1 == i2 || i2 == i3 || i3 == i1 {
                    continue
                }
                if x + y + z == 2020 {
                    return Some(x * y * z)
                }
            }
        }
    }
    None
}

pub fn day01() -> String {
    let entries = lines_from!("01", u64);
    let solution = match solve_day01_1(entries) {
        Some(answer) => format!("{}", answer),
        None => thats_weird()
    };
    format!("Day 1-1:  {}", solution)
}

pub fn day01_part2() -> String {
    let entries = lines_from!("01", u64);
    let solution = match solve_day01_2(entries) {
        Some(answer) => format!("{}", answer),
        None => thats_weird()
    };
    format!("Day 1-2:  {}", solution)

}

#[test]
pub fn solve_day01_test() {
    let mut entries: Vec<u64> = vec![1010, 1010, 999, 333];
    assert_eq!(Some(1020100), solve_day01_1(entries));

    entries = vec![0, 0];
    assert_eq!(None, solve_day01_1(entries));

    entries = vec![1011, 1010, 313, 312];
    assert_eq!(None, solve_day01_1(entries));
}

#[test]
pub fn solve_day01_part2_test() {
    let entries: Vec<u64> = vec![383, 300, 1337, 333];
    assert_eq!(Some(153621300), solve_day01_2(entries));

    let second_entries = vec![0, 0, 0];
    assert_eq!(None, solve_day01_2(second_entries));

    let third_entries = vec![384, 300, 1337, 333];
    assert_eq!(None, solve_day01_2(third_entries));

}
