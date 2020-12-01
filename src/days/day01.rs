use crate::utils::thats_weird;

fn solve_day01_1(input: Vec<i32>) -> Option<i32> {
    let mut entries = input.clone();
    entries.sort_unstable_by(|a, b| b.cmp(a));
    for x in entries.iter() {
        if entries.contains(&(2020-x)) {
            return Some(x * (2020-x))
        }
    }
    None
}

fn solve_day01_2(input: Vec<i32>) -> Option<i32> {
    let mut entries = input.clone();
    entries.sort_unstable();
    for x in entries.iter() {
        for y in entries.iter() {
            if entries.contains(&(2020-(x+y))) {
                return Some(x*y*&(2020-(x+y)))
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
    // these tests don't really test what i want, but the actual case seems to fly with the implementation
    // so whatever.
    let mut entries: Vec<i32> = vec![1011, 1009, 999, 333];
    assert_eq!(Some(1020099), solve_day01_1(entries));

    entries = vec![0, 0];
    assert_eq!(None, solve_day01_1(entries));

    entries = vec![1011, 1011, 313, 312];
    assert_eq!(None, solve_day01_1(entries));
}

#[test]
pub fn solve_day01_part2_test() {
    let entries: Vec<i32> = vec![383, 300, 1337, 333];
    assert_eq!(Some(153621300), solve_day01_2(entries));

    let second_entries = vec![0, 0, 0];
    assert_eq!(None, solve_day01_2(second_entries));

    let third_entries = vec![384, 300, 1337, 333];
    assert_eq!(None, solve_day01_2(third_entries));

}
