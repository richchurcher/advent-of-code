use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn converter (input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim_start_matches("+").parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn calibrate (deltas: &[i32]) -> i32 {
    deltas
        .iter()
        .fold(0, |total, d| total + d)
}

#[aoc(day1, part2)]
pub fn calibrate_until_match (deltas: &[i32]) -> i32 {
    let mut unique_freqs = HashSet::new();
    let mut freq = 0;

    loop {
        for delta in deltas {
            freq += delta;
            let unique = unique_freqs.insert(freq);
            if unique == false {
                return freq;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converter_removes_plus_symbols_and_returns_integers () {
        assert_eq!(converter("+1\n-2\n+3\n"), [1, -2, 3])
    }

    #[test]
    fn calibrate_sums_a_vec_of_ints () {
        assert_eq!(calibrate(&vec![1, -2, 3]), 2)
    }

    #[test]
    fn calibrate_progressive_returns_progressive_sum () {
        assert_eq!(calibrate_progressive(&vec![1, -2, 3]), [1, -1, 2])
    }

    #[test]
    fn calibrate_until_match_finds_the_first_matching_value () {
        assert_eq!(calibrate_until_match(&vec![1, -2, 3, 1]), 2)
    }
}
