use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let cases = read_lines("src/input.txt");
    let mut total_sum = 0;

    let mapping = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);

    let keys_set: HashSet<&str> = mapping.keys().cloned().collect();
    let values_set: HashSet<&str> = mapping.values().cloned().collect();
    let combined_set: HashSet<&str> = keys_set.union(&values_set).cloned().collect();

    for case in &cases {
        let mut first = find_first_occurrence(&combined_set, case);
        let mut last = find_last_occurrence(&combined_set, case);

        if let Some(first_str) = first {
            if keys_set.contains(first_str) {
                first = mapping.get(first_str).copied();
            }
        }

        if let Some(last_str) = last {
            if keys_set.contains(last_str) {
                last = mapping.get(last_str).copied();
            }
        }

        let sum = match (first, last) {
            (Some(s1), Some(s2)) => Some(format!("{}{}", s1, s2).parse::<i32>()),
            _ => None,
        };

        let sum_as_i32 = match sum {
            Some(Ok(value)) => value,
            _ => -1,
        };

        total_sum = total_sum + sum_as_i32
    }

    println!("total sum: {}", total_sum);
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
