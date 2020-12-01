use crate::utils::thats_weird;

fn solve_day01_1(input: Vec<i32>) -> Option<i32> {
    let mut entries = input.clone();
    entries.sort_unstable_by(|a, b| b.cmp(a));
    for (i, x) in entries.iter().enumerate() {
        let mut relevant_entries = entries.clone();
        relevant_entries.remove(i);
        if relevant_entries.contains(&(2020-x)) {
            return Some(x * (2020-x))
        }
    }
    None
}

fn solve_day01_2(input: Vec<i32>) -> Option<i32> {
    let mut entries = input.clone();
    entries.sort_unstable();
    for (i1, x) in entries.iter().enumerate() {
        for (i2, y) in entries.iter().enumerate() {
            if i1 == i2 {
                continue
            }
            let mut relevant_entries = entries.clone();
            if i1 > i2 {
                relevant_entries.remove(i1);
                relevant_entries.remove(i2);
            } else {
                relevant_entries.remove(i2);
                relevant_entries.remove(i1);
            }
            if relevant_entries.contains(&(2020-x-y)) {
                return Some(x * y * &(2020-x-y))
            }
        }
    }
    None
}

pub fn day01() -> String {
    let entries = lines_from!("01", i32);
    let answer = match solve_day01_1(entries) {
        Some(solution) => format!("{}", solution),
        None => thats_weird()
    };
    format!("Day 1-1:  {}", answer)
}

pub fn day01_part2() -> String {
    let entries = lines_from!("01", i32);
    let answer = match solve_day01_2(entries) {
        Some(solution) => format!("{}", solution),
        None => thats_weird()
    };
    format!("Day 1-2:  {}", answer)

}

#[test]
pub fn solve_day01_test() {
    let mut entries: Vec<i32> = vec![0, 0];
    assert_eq!(None, solve_day01_1(entries));

    entries = vec![1011, 1010, 313, 312];
    assert_eq!(None, solve_day01_1(entries));

    entries = vec![1010, 1010, 999, 333];
    assert_eq!(Some(1020100), solve_day01_1(entries));
}

#[test]
pub fn solve_day01_part2_test() {
    let mut entries: Vec<i32> = vec![0, 0, 0];
    assert_eq!(None, solve_day01_2(entries));

    entries = vec![1008, 4, 1337, 333];
    assert_eq!(None, solve_day01_2(entries));

    entries = vec![1008, 4, 1008, 333];
    assert_eq!(Some(4064256), solve_day01_2(entries));

    entries = vec![382, 300, 1337, 333];
    assert_eq!(None, solve_day01_2(entries));

    entries = vec![383, 300, 1337, 333];
    assert_eq!(Some(153621300), solve_day01_2(entries));
}
