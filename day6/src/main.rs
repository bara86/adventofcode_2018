use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;
use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut coordinates = Vec::new();

    let mut x_min = i32::max_value();
    let mut x_max = i32::min_value();
    let mut y_min = x_min;
    let mut y_max = x_max;

    for line in reader.lines() {
        let line = line?;

        let mut coords = line.split(", ");
        let x = coords.next().unwrap().parse::<i32>().unwrap();
        let y = coords.next().unwrap().parse::<i32>().unwrap();

        coordinates.push((x, y));

        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);

    }

    let mut matrix = HashMap::new();
    let mut cattivoni = HashSet::new();

    dbg!((x_min, x_max, y_min, y_max));
    let mut numeri = 0;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let mut best = i32::max_value();
            let mut idx = None;

            let mut sum = 0;

            for (i, coords) in coordinates.iter().enumerate() {
                let dist = (coords.0 - x).abs() + (coords.1 - y).abs();
                sum += dist;

                if dist < best {
                    best = dist;
                    idx = Some(i);
                } else if dist == best {
                    idx = None;
                }
            }

            if let Some(idx) = idx {
                let coords = coordinates[idx];
                if x == x_min || x == x_max || y == y_min || y == y_max {
                    cattivoni.insert(idx);
                }
                *matrix.entry(idx).or_insert(0) += 1;
            }

            if sum < 10000 {
                numeri += 1;
            }
        }
    }

    println!("{:?}", matrix.iter().filter(|(idx, v)| !cattivoni.contains(idx)).max_by_key(|(idx, v)| **v));
    println!("{}", numeri);

    Ok(())
}