use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug)]
struct Event {
    time: String,
    event: String,
}

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut events = Vec::new();

    let date_regex = Regex::new(r"(\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}])").unwrap();
    let event_regex = Regex::new(r"] (.*)$").unwrap();
    let guard_number_regex = Regex::new(r"#(\d+)").unwrap();
    let minute_regex = Regex::new(r":(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;
        let time = date_regex
            .captures(&line)
            .unwrap()
            .get(0)
            .map(|x| x.as_str().to_string())
            .unwrap();
        let event = event_regex
            .captures(&line)
            .unwrap()
            .get(1)
            .map(|x| x.as_str().to_string())
            .unwrap();

        events.push(Event { time, event });
    }

    events.sort_by(|first, second| first.time.cmp(&second.time));

    let mut sleepy_times = HashMap::new();
    let mut minutes_in_sleep = HashMap::new();

    let mut guard = String::new();
    let mut start_sleep_time = 0;
    let mut end_sleep_time: i32;

    for event in events.iter() {
        if event.event == "falls asleep" {
            start_sleep_time = minute_regex
                .captures(&event.time)
                .unwrap()
                .get(1)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .unwrap();
        } else if event.event == "wakes up" {
            end_sleep_time = minute_regex
                .captures(&event.time)
                .unwrap()
                .get(1)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .unwrap();

            *sleepy_times.entry(guard.clone()).or_insert(0) += end_sleep_time - start_sleep_time;

            let k = minutes_in_sleep
                .entry(guard.clone())
                .or_insert(HashMap::new());

            for i in start_sleep_time..end_sleep_time {
                *k.entry(i).or_insert(0) += 1;
            }
        } else {
            guard = guard_number_regex
                .captures(&event.event)
                .unwrap()
                .get(1)
                .map(|x| x.as_str().to_string())
                .unwrap();
        }
    }

    let sleepiest_guard = sleepy_times.iter().max_by_key(|(_, m)| *m).unwrap().0;

    let sleep_time = &minutes_in_sleep[sleepiest_guard];
    let minute_guard_slept_most = sleep_time.iter().max_by_key(|(_, m)| *m).unwrap().0;

    println!("{:?} {}", minute_guard_slept_most, sleepiest_guard);

    let vv = minutes_in_sleep
        .iter()
        .flat_map(|(guard, mins)| mins.iter().map(move |m| (guard, m)))
        .max_by_key(|(_, m)| *m.1)
        .unwrap();

    println!("{:?}, {:?}", vv.0, vv.1);

    Ok(())
}