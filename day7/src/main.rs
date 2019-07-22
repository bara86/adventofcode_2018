use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;
use std::collections::{HashMap, HashSet, BTreeMap};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    // Part 1
    let mut dependencies = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;

        let mut splitto = line.split(' ');
        let dep = splitto.nth(1).unwrap().chars().next().unwrap();
        let addicted = splitto.nth(5).unwrap().chars().next().unwrap();

        dependencies.entry(addicted).or_insert(HashSet::new()).insert(dep);
        dependencies.entry(dep).or_insert(HashSet::new());

    }

    let mut dependencies2 = dependencies.clone();

    let mut combination = String::new();
    while !dependencies.is_empty() {
        let first = dependencies.iter().find(|(key, value)| value.is_empty()).unwrap().0.clone();

        dependencies.remove(&first);
        combination.push(first);

        for values in dependencies.values_mut() {
            values.remove(&first);
        }

    }

    println!("{}", combination);

    // Part 2

    let mut workers = vec![None; 5];
    let mut elapsed_time = 0;

    while !dependencies2.is_empty() || workers.iter().any(|x| x.is_some()) {
        for worker in workers.iter_mut() {
            if worker.is_some() {
                continue;
            }

            if let Some(first) = dependencies2.iter().find(|(key, value)| value.is_empty()).map(|(k, v)| *k) {
                *worker = Some((first, first as usize - 'A' as usize + 61));
                dependencies2.remove(&first);
            }
        }

        let minimum = workers.iter().filter_map(|item| item.as_ref()).min_by_key(|x| x.1).unwrap().1.clone();

        for worker in workers.iter_mut() {
            if let Some(w) = worker {
                w.1 -= minimum;
                if w.1 == 0 {
                    for values in dependencies2.values_mut() {
                        values.remove(&w.0);
                    }

                    *worker = None;
                }
            }
        }

        elapsed_time += minimum;

    }

    println!("{}", elapsed_time);

    Ok(())
}