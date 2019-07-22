use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut polymers = String::new();

    reader.read_line(&mut polymers)?;

    let best = (b'a'..b'z').map(|c_lower| {
        let c_lower = c_lower as char;
        let c_upper = c_lower.to_ascii_uppercase();

        reaction(polymers.chars().filter(|c| *c != c_lower && *c != c_upper)).len()
    }).min();

    println!("{:?}", best);

    Ok(())
}

fn reaction(polymers: impl Iterator<Item=char>) -> String {
    let mut new_string = String::new();

    for c in polymers {
        if let Some(last_char) = new_string.chars().last() {
            if (last_char as i32 - c as i32).abs() == 32 {
                new_string.pop();
                continue
            }
        }

        new_string.push(c);
    }

    new_string
}
