#![allow(non_snake_case)]

use std::collections::HashSet;

#[aoc(day5, part1)]
fn find_stable (input: &str) -> usize {
    let polymer: Vec<char> = input.chars().collect();
    let mut stable = reaction(&polymer);
    let mut this_count = stable.iter().count();
    let mut last_count = 0;

    while this_count != last_count {
        stable = reaction(&stable);
        last_count = this_count;
        this_count = stable.iter().count();
    }
    this_count
}

#[aoc(day5, part2)]
fn find_improved_stable (input: &str) -> usize {
    let units = find_unit_types(input);
    let mut best_length = input.len();

    for unit in units.iter() {
        let polymer_without: String = input
            .chars()
            .filter(|c| *unit != c.to_lowercase().next().unwrap())
            .collect();
        let length = find_stable(polymer_without.as_str());
        if length < best_length {
            best_length = length;
        }
    }
    best_length
}

fn reaction (polymer: &Vec<char>) -> Vec<char> {
    let mut stable: Vec<char> = vec![];

    let mut c = polymer.iter().peekable();
    let mut skip_char = false;

    while let Some(first) = c.next() {
        if skip_char {
            skip_char = false;
            continue;
        }

        match c.peek() {
            Some(second) => {
                if will_react(first, second) {
                    skip_char = true;
                } else {
                    stable.push(*first);
                }
            },
            None => stable.push(*first),
        }
    }

    stable
}

fn will_react (first: &char, second: &char) -> bool {
    let up_first = first.to_ascii_uppercase();
    let up_second = second.to_ascii_uppercase();

    if up_first != up_second {
        return false;
    }

    if first.is_uppercase() && second.is_uppercase() {
        return false;
    }

    if first.is_lowercase() && second.is_lowercase() {
        return false;
    }

    true
}

fn find_unit_types (polymer: &str) -> Vec<char> {
    let mut types = HashSet::<char>::new();
    let chars: Vec<char> = polymer.chars().collect();
    for c in chars.into_iter() {
        types.insert(c.to_lowercase().next().unwrap());
    }
    types.into_iter().collect()
}

#[cfg(test)]

    #[test]
    fn will_react_returns_true_for_aA () {
        assert_eq!(will_react(&'a', &'A'), true)
    }

    #[test]
    fn will_react_returns_true_for_Aa () {
        assert_eq!(will_react(&'A', &'a'), true)
    }

    #[test]
    fn will_react_returns_false_for_aB () {
        assert_eq!(will_react(&'a', &'B'), false)
    }

    #[test]
    fn will_react_returns_false_for_Ba () {
        assert_eq!(will_react(&'B', &'a'), false)
    }

    #[test]
    fn will_react_returns_false_for_AA () {
        assert_eq!(will_react(&'A', &'A'), false)
    }

    #[test]
    fn will_react_returns_false_for_AB () {
        assert_eq!(will_react(&'A', &'B'), false)
    }

    #[test]
    fn will_react_returns_false_for_aa () {
        assert_eq!(will_react(&'a', &'a'), false)
    }

    #[test]
    fn reaction_correct_for_one_pass_over_sample_data () {
        assert_eq!(
            reaction(&"dabAcCaCBAcCcaDA".chars().collect()),
            vec!['d', 'a', 'b', 'A', 'a', 'C', 'B', 'A', 'c', 'a', 'D', 'A']
        )
    }

    #[test]
    fn find_stable_correct_for_sample_data () {
        assert_eq!(find_stable(&"dabAcCaCBAcCcaDA"), 10)
    }

    #[test]
    fn find_unit_types_correct_for_small_sample () {
        let mut actual = find_unit_types(&"dabAacCaCbAa");
        actual.sort_unstable();
        assert_eq!(actual, vec!['a', 'b', 'c', 'd'])
    }

    #[test]
    fn find_improved_stable_correct_for_sample_data () {
        assert_eq!(find_improved_stable("dabAcCaCBAcCcaDA"), 4)
    }

