use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[aoc_generator(day4)]
fn populate_events (input: &str) -> HashMap<String, HashMap<String, u32>> {
    let records = sort_by_datetime(input);
    let mut guards = HashMap::new();
    let mut guard = String::from("");
    let mut sleep_time = String::from("");

    for record in records.iter() {
        let parts: Vec<&str> = record.split(" ").collect();
        let action = parts[2];

        match action {
            "Guard" => {
                guard = parts[3].to_string();
                guards.entry(guard.clone()).or_insert(HashMap::new());
            },
            "falls" => {
                sleep_time = parts[1].to_string();
                sleep_time.pop();
                let times = guards.entry(guard.clone());
                match times {
                    Entry::Occupied(t) => {
                        *t.into_mut().entry(sleep_time).or_insert(0) += 1;
                    },
                    // Shouldn't be possible to reach
                    Entry::Vacant(_) => continue,
                }
            },
            "wakes" => {
                
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

#[aoc(day4, part1)]
fn check (guards: &HashMap<String, HashMap<String, u32>>) -> u32 {
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
            .insert("00:30".to_string(), 1);
        expected.entry("#10".to_string())
            .or_insert_with(HashMap::new)
            .insert("00:05".to_string(), 1);

        let unsorted = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:31] wakes up";
        assert_eq!(populate_events(unsorted), expected)
    }
