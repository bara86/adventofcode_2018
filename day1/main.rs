use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Read;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {

    let f = fs::File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let sum = reader.by_ref().lines().map(|x| x.unwrap().parse::<i32>().unwrap()).sum::<i32>();
    println!("{}", sum);

    reader.seek(SeekFrom::Start(0))?;

    let mut values = HashSet::new();

    let mut sum = 0;

    for value in reader.lines().map(|x| x.unwrap().parse::<i32>().unwrap()).collect::<Vec<_>>().into_iter().cycle() {
        sum += value;

        if !values.insert(sum) {
            println!("Trovato {}", sum);
            break;
        }
    }

    return Ok(());

}