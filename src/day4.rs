use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[aoc_generator(day4)]
fn parse_log (input: &str) -> HashMap<u32, HashMap<u32, u32>> {
    let records = sort_by_datetime(input);
    let mut guards = HashMap::new();
    let mut guard = 0;
    let mut sleep_time = 0;

    for record in records.iter() {
        let parts: Vec<&str> = record.split(" ").collect();
        let action = parts[2];

        match action {
            "Guard" => {
                guard = parts[3][1..].parse().unwrap();
                guards.entry(guard).or_insert_with(HashMap::new);
            },
            "falls" => {
                sleep_time = parse_time(parts[1]);
                increment_time(&mut guards, guard, sleep_time);
            },
            "wakes" => {
                let wake_time: u32 = parse_time(parts[1]);
                sleep_time += 1;

                for minute in sleep_time..=wake_time {
                    increment_time(&mut guards, guard, minute);
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

fn increment_time (guards: &mut HashMap<u32, HashMap<u32, u32>>, guard: u32, time: u32) {
    let times = guards.entry(guard);
    match times {
        Entry::Occupied(t) => {
            *t.into_mut().entry(time).or_insert(0) += 1;
        },
        _ => unreachable!(),
    }
}

#[aoc(day4, part1)]
fn find_sleepiest (guards: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let mut total_sleepy_minutes = 0;
    let mut sleepiest_guard = 0;
    let mut sleepiest_minute = 0;

    for (id, times) in guards.iter() {
        let minutes = times.iter().fold(0, |total, (_, v)| total + v);
        if minutes > total_sleepy_minutes {
            total_sleepy_minutes = minutes;
            sleepiest_guard = *id;

            let mut highest_count = 0;
            for (time, n) in times.iter() {
                if *n > highest_count {
                    sleepiest_minute = *time;
                    highest_count = *n;
                }
            }
        }
    }
    sleepiest_guard * sleepiest_minute
}

#[aoc(day4, part2)]
fn find_sleepiest_minute_overall (guards: &HashMap<u32, HashMap<u32, u32>>) -> u32 {
    let mut sleepiest_guard = 0;
    let mut sleepiest_minute = 0;
    let mut sleep_count = 0;

    for (id, times) in guards.iter() {
        for (time, n) in times.iter() {
            if *n > sleep_count {
                sleep_count = *n;
                sleepiest_minute = *time;
                sleepiest_guard = *id;
            }
        }
    }
    println!("#{}: {} ({})", sleepiest_guard, sleepiest_minute, sleep_count);
    sleepiest_guard * sleepiest_minute
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
        expected.entry(10).or_insert_with(HashMap::new).insert(30, 1);
        expected.entry(10).or_insert_with(HashMap::new).insert(31, 1);
        expected.entry(10).or_insert_with(HashMap::new).insert(5, 1);
        expected.entry(10).or_insert_with(HashMap::new).insert(6, 1);
        expected.entry(10).or_insert_with(HashMap::new).insert(7, 1);

        let unsorted = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:07] wakes up\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:31] wakes up";
        assert_eq!(parse_log(unsorted), expected)
    }

    #[test]
    fn find_sleepiest_minute_overall_correct_with_sample_data () {
        let sample = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up\n";
        assert_eq!(find_sleepiest_minute_overall(&parse_log(&sample)), 0)
    }
