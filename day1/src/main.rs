fn main() {
    let cases = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    for case in cases {
        get_first_and_last_digit(String::from("case"))
    }
}

fn get_first_and_last_digit(s: String) {
    let chars = s.chars();
    for c in chars {
        if c.is_ascii_digit() {
            println!("{}", c)
        } else {
            println!("Not a number")
        }
    }
}
