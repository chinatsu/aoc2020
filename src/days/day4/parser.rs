use super::{PassportQueue, Passport};
use std::io::BufRead;

pub fn parse(filename: &str) -> PassportQueue {
    let file = std::fs::File::open(format!("src/days/day4/resources/{}.txt", filename)).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut buffer: Vec<String> = Vec::new();
    let mut entry: String = String::new();
    for val in reader.lines() {
        let line = val.unwrap();
        if line.len() == 0 {
            buffer.push(entry);
            entry = String::new();
        }
        entry = format!("{} {}", entry, line);
    }
    buffer.push(entry);
    buffer.iter().map(|val| {
        parse_entry(val)
    }).collect::<PassportQueue>()
}

fn parse_entry(entries: &String) -> Passport {
    let entries = entries.split(' ').map(|v| v.split(':').collect()).collect::<Vec<Vec<&str>>>();
    let mut ret = Passport {
        byr: String::new(),
        iyr: String::new(),
        eyr: String::new(),
        hgt: String::new(),
        hcl: String::new(),
        ecl: String::new(),
        pid: String::new()
    };
    for entry in entries {
        match entry[0] {
            "byr" => { ret.byr = entry[1].to_string() },
            "iyr" => { ret.iyr = entry[1].to_string() },
            "eyr" => { ret.eyr = entry[1].to_string() },
            "hgt" => { ret.hgt = entry[1].to_string() },
            "hcl" => { ret.hcl = entry[1].to_string() },
            "ecl" => { ret.ecl = entry[1].to_string() },
            "pid" => { ret.pid = entry[1].to_string() },
            _ => {}
        }
    }
    ret
}

#[test]
fn parse_test() {
    let input = parse("test");
    assert_eq!("1937", input[0].byr);
    assert_eq!("166559648", input[3].pid);
}

