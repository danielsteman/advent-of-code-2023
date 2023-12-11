use std::fs::read_to_string;

fn main() {
    let cases = read_lines("src/input.txt");
    let mut total_sum = 0;

    for case in &cases {
        if let Some(digits) = get_first_and_last_digit(String::from(case)) {
            println!("first digit: {}", digits.first_digit);
            println!("last digit: {}", digits.last_digit);
            let result = digits.sum();
            match result {
                Ok(sum) => total_sum = total_sum + sum,
                Err(_) => println!("couldn't parse sum"),
            }
        } else {
            println!("no first and/or last digit found")
        }
    }

    println!("total sum: {}", total_sum)

    // let set = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    // let s = "hoihoionehoitwohoifour";
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
