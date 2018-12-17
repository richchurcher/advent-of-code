use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[aoc_generator(day4)]
fn populate_events (input: &str) -> HashMap<String, HashMap<u32, u32>> {
    let records = sort_by_datetime(input);
    let mut guards = HashMap::new();
    let mut guard = String::new();
    let mut sleep_time = 0;

    for record in records.iter() {
        let parts: Vec<&str> = record.split(" ").collect();
        let action = parts[2];

        match action {
            "Guard" => {
                guard = parts[3].to_string();
                guards.entry(guard.clone()).or_insert_with(HashMap::new);
            },
            "falls" => {
                sleep_time = parse_time(parts[1]);
                increment_time(&mut guards, &guard, sleep_time);
            },
            "wakes" => {
                let wake_time: u32 = parse_time(parts[1]);
                sleep_time += 1;

                for minute in sleep_time..=wake_time {
                    increment_time(&mut guards, &guard, minute);
                }
            },
            _ => continue,
        }
    }
    guards
}

fn sort_by_datetime (input: &str) -> Vec<&str> {
    let mut records: Vec<&str> = input.lines().collect();
    records.sort();
    records
}

fn parse_time (raw_time: &str) -> u32 {
    let mut time = raw_time[3..].to_string();
    time.pop();
    time.parse().unwrap()
}

fn increment_time (guards: &mut HashMap<String, HashMap<u32, u32>>, guard: &String, time: u32) {
    let times = guards.entry(guard.to_string());
    match times {
        Entry::Occupied(t) => {
            *t.into_mut().entry(time).or_insert(0) += 1;
        },
        _ => unreachable!(),
    }
}

#[aoc(day4, part1)]
fn check (guards: &HashMap<String, HashMap<u32, u32>>) -> u32 {
    println!("{:?}", guards);
    0
}

#[cfg(test)]

    #[test]
    fn sort_by_datetime_sorts_asc () {
        let expected = vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
        ];
        let unsorted = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:30] falls asleep";
        assert_eq!(sort_by_datetime(unsorted), expected)
    }

    #[test]
    fn generator_output_correct () {
        let mut expected = HashMap::new();

        // "[1518-11-01 00:00] Guard #10 begins shift",
        // "[1518-11-01 00:05] falls asleep",
        // "[1518-11-01 00:25] wakes up",
        // "[1518-11-01 00:30] falls asleep",
        // "[1518-11-01 00:31] wakes up",
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert(30, 1);
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert(31, 1);
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert(5, 1);
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert(6, 1);
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert(7, 1);

        let unsorted = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:07] wakes up\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:31] wakes up";
        assert_eq!(populate_events(unsorted), expected)
    }
