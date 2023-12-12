use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let cases = read_lines("src/input.txt");
    let mut total_sum = 0;

    for case in &cases {
        if let Some(digits) = get_first_and_last_digit(String::from(case)) {
            // println!("first digit: {}", digits.first_digit);
            // println!("last digit: {}", digits.last_digit);
            let result = digits.sum();
            match result {
                Ok(sum) => total_sum = total_sum + sum,
                Err(_) => println!("couldn't parse sum"),
            }
        } else {
            // println!("no first and/or last digit found")
        }
    }

    println!("total sum: {}", total_sum);

    let set: HashSet<&str> = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "zero",
    ]
    .iter()
    .cloned()
    .collect();

    let s = "sdgionsodgn9asfoiafknasfpas0asfpiasjhf8";
    // println!("{:?}", s.rfind("two"))
    println!("{:?}", find_first_occurrence(&set, s));
    println!("{:?}", find_last_occurrence(&set, s));
}

fn find_first_occurrence<'a>(set: &HashSet<&'a str>, input_string: &'a str) -> Option<&'a str> {
    let mut earliest_index: Option<usize> = None;
    let mut earliest_value: Option<&str> = None;

    for value in set.iter() {
        if let Some(index) = input_string.find(value) {
            match earliest_index {
                Some(cur_earliest_idx) if index < cur_earliest_idx => {
                    earliest_index = Some(index);
                    earliest_value = Some(value);
                }
                None => {
                    earliest_index = Some(index);
                    earliest_value = Some(value);
                }
                _ => {}
            }
        }
    }

    earliest_value
}

fn find_last_occurrence<'a>(set: &HashSet<&'a str>, input_string: &'a str) -> Option<&'a str> {
    let mut last_index: Option<usize> = None;
    let mut last_value: Option<&str> = None;

    for value in set.iter() {
        if let Some(index) = input_string.rfind(value) {
            match last_index {
                Some(cur_last_idx) if index > cur_last_idx => {
                    last_index = Some(index);
                    last_value = Some(value);
                }
                None => {
                    last_index = Some(index);
                    last_value = Some(value);
                }
                _ => {}
            }
        }
    }

    last_value
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

struct FirstLastDigits {
    first_digit: i32,
    last_digit: i32,
}

impl FirstLastDigits {
    fn sum(&self) -> Result<i32, std::num::ParseIntError> {
        let concatenated = format!("{}{}", &self.first_digit, &self.last_digit);
        let sum = concatenated.parse::<i32>()?;
        Ok(sum)
    }
}

fn get_first_and_last_digit(s: String) -> Option<FirstLastDigits> {
    let first_digit = s.chars().clone().find(|c| c.is_ascii_digit())?;
    let last_digit = s.chars().clone().rev().find(|c| c.is_ascii_digit())?;

    Some(FirstLastDigits {
        first_digit: first_digit as i32 - '0' as i32,
        last_digit: last_digit as i32 - '0' as i32,
    })
}
