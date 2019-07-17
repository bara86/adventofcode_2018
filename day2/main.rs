use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let f = fs::File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut strings = vec![];

    let mut count_2 = 0;
    let mut count_3 = 0;

     for line in reader.lines() {
         let line = line.unwrap();

         let mut sums = HashMap::new();

         for c in line.chars() {
             *sums.entry(c).or_insert(0) += 1;
         }

         if sums.values().any(|v| *v == 2) {
             count_2 += 1;
         }
         if sums.values().any(|v| *v == 3) {
             count_3 += 1;
         }

         strings.push(line);
     }

     println!("{}", count_2 * count_3);

     for (idx, value) in strings.iter().enumerate() {
        for value2 in strings.iter().skip(idx) {
            if value.chars().zip(value2.chars()).filter(|(v1, v2)| v1 != v2).count() == 1 {
                println!("{}", value.chars().zip(value2.chars()).filter(|(v1, v2)| v1 == v2).map(|(v1, v2)| v1).collect::<String>());
            }
        }
     }

     Ok(())
}