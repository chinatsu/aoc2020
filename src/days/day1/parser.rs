use std::io::BufRead;

pub fn parse(filename: &str) -> Vec<i32> {
    let file = std::fs::File::open(format!("src/days/day1/resources/{}.txt", filename)).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|val| val.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn parse_test() {
    assert_eq!(vec![1, 2, 3], parse("test"))
}
