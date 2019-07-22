use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut polymers = String::new();

    reader.read_line(&mut polymers)?;

    let mut new_string = String::new();

    for c in polymers.chars() {
        if let Some(last_char) = new_string.chars().last() {
            if (last_char as i32 - c as i32).abs() == 32 {
                new_string.pop();
                continue
            }
        }

        new_string.push(c);
    }

    println!("{}", new_string.len());

    Ok(())
}
