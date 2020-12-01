#[macro_export]
macro_rules! lines_from {
    ($filename:expr, $type:tt) => {
        {
            use std::io::BufRead;
            let file = std::fs::File::open(format!("days/{}.txt", $filename)).unwrap();
            let reader = std::io::BufReader::new(file);
            reader.lines().map(|val| val.unwrap().parse::<$type>().unwrap())
                .collect::<Vec<$type>>()
        }
    }
}

#[macro_export]
macro_rules! csv_from {
    ($filename:expr, $type:tt) => {
        std::fs::read_to_string(format!("days/{}.txt", $filename))
            .unwrap()
            .trim()
            .split(",")
            .map(|val| val.trim().parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
        }
}

#[test]
fn lines_from_test1_should_be_1_2_3() {
    assert_eq!(vec![1, 2, 3], lines_from!("test1", u32))
}

#[test]
fn csv_from_test2_should_be_65535_65536() {
    assert_eq!(vec![65535, 65536], csv_from!("test2", u32))
}
