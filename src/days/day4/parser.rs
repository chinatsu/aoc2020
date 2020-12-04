use super::{PassportQueue, Passport};

pub fn parse(filename: &str) -> PassportQueue {
    let file = std::fs::read_to_string(format!("src/days/day4/resources/{}.txt", filename)).unwrap();
    file.split("\n\n").map(|entry| {
        parse_entry(&entry)
    }).collect::<PassportQueue>()
}

fn parse_entry(entries: &str) -> Passport {
    let entries = entries
        .split(&[' ', '\n'][..])
        .map(|v| v.split(':').collect())
        .collect::<Vec<Vec<&str>>>();
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

