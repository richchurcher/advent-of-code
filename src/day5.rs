#![allow(non_snake_case)]

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
        assert_eq!(find_stable("dabAcCaCBAcCcaDA"), 10)
    }
