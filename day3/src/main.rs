use regex::Regex;
use std::fs;
fn main() {
    let path = "data/input";
    let data = fs::read_to_string(path).unwrap();
    let data = data.replace('\n', " ");
    let operations = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();

    let mut flag = true;

    let result: u32 = operations
        .captures_iter(&data)
        .map(|c| match &c[0] {
            "do()" => {
                flag = true;
                return 0;
            }
            "don't()" => {
                flag = false;
                return 0;
            }
            _ => c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap() * (flag as u32),
        })
        .sum();

    dbg!(&result);
}
