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
            .map(|val| val.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
        }
}
