extern crate regex;

use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::string::String;
use regex::Regex;

#[derive(Debug)]
struct Event {
    time: String,
    event: String,
}


fn main() -> std::io::Result<()> {
    let f = fs::File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut events = Vec::new();

    let date_regex = Regex::new(r"(\[\d{4}-\d{2}-\d{2}\s\d{2}:\d{2}])").unwrap();
    let event_regex = Regex::new(r"] (.*)$").unwrap();
    let guard_number_regex = Regex::new(r"#(\d+)").unwrap();
    let minute_regex = Regex::new(r":(\d+)").unwrap();

    for line in reader.lines() {
        let line = line?;
        let time = date_regex.captures(&line).unwrap().get(0).map(|x| x.as_str()).unwrap().to_string();
        let event = event_regex.captures(&line).unwrap().get(1).map(|x| x.as_str()).unwrap().to_string();
        
        let ev = Event {time, event};
        events.push(ev);

    }

    events.sort_by(|first, second| first.time.cmp(&second.time));

    let mut sleepy_times = HashMap::new();
    let mut minutes_in_sleep = HashMap::new();

    let mut guard = String::new();
    let mut start_sleep_time = 0;
    let mut end_sleep_time: i32;

    for event in events.iter() {
        if guard_number_regex.is_match(event.event.as_str()) {
            guard = guard_number_regex.captures(&event.event).unwrap().get(1).map(|x| x.as_str()).unwrap().to_string();
        }
        else if event.event == "falls asleep" {
            start_sleep_time = minute_regex.captures(&event.time).unwrap().get(1).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();
        }
        else {
            assert_eq!(event.event, "wakes up");
            end_sleep_time = minute_regex.captures(&event.time).unwrap().get(1).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();

            *sleepy_times.entry(guard.as_str().to_string()).or_insert(0) += end_sleep_time - start_sleep_time;

            let tmp = String::from(guard.as_str());
            let def: HashMap<i32, i32> = HashMap::new();
            let k = minutes_in_sleep.entry(tmp).or_insert(def);

            for i in start_sleep_time..end_sleep_time {
                *k.entry(i).or_insert(0) += 1;
            }
        }
    }

    let sleepiest_guard = sleepy_times.iter().max_by_key(|x| x.1).unwrap().0;

    let sleep_time = minutes_in_sleep.get(sleepiest_guard).unwrap();
    let minute_guard_slept_most = sleep_time.iter().max_by_key(|x| x.1).unwrap().0;

    println!("{:?} {}", minute_guard_slept_most, sleepiest_guard);

    Ok(())
}