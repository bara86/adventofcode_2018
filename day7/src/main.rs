use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;
use std::collections::{HashMap, HashSet, BTreeMap};

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut dependencies = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;

        let mut splitto = line.split(' ');
        let dep = splitto.nth(1).unwrap().chars().next().unwrap();
        let addicted = splitto.nth(5).unwrap().chars().next().unwrap();

        dependencies.entry(addicted).or_insert(HashSet::new()).insert(dep);
        dependencies.entry(dep).or_insert(HashSet::new());

    }

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

    Ok(())
}