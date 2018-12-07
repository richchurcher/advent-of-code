use std::collections::HashSet;

pub fn has_n_dups (id: &str, n_dups: usize) -> bool {
    let mut unique_chars = HashSet::new();

    for c in id.chars() {
        let unique = unique_chars.insert(c);
        if !unique {
          if id.matches(c).count() == n_dups {
              return true;
          }
        }
    }
    false
}

#[aoc(day2, part1)]
pub fn find_checksum (ids: &str) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids.lines() {
        if has_n_dups(id, 2) {
            twos += 1;
        }
        if has_n_dups(id, 3) {
            threes += 1;
        }
    }
    twos * threes
}
pub fn compare_ids (a: &str, b: &str) -> bool {
    let similar = a.chars().zip(b.chars())
        .filter(|&(a, b)| a == b)
        .count();
    similar == a.chars().count() - 1
}


pub fn find_similar_ids<'a> (needle: &str, haystack: &'a [&str]) -> Option<&'a str> {
    for id in haystack.iter() {
        if compare_ids(needle, id) {
            return Some(id)
        }
    }
    None
}

#[aoc(day2, part2)]
pub fn find_common_letters (input: &str) -> Option<String> {
    let ids = input
        .lines()
        .collect::<Vec<_>>();
    for (i, id) in ids.iter().enumerate() {
        let next_index = i + 1;
        let similar = match find_similar_ids(id, &ids[next_index..]) {
            Some(s) => s,
            None => continue,
        };
        let common = id.chars()
            .zip(similar.chars())
            .filter(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();
        return Some(common);
    }
    None
}

#[cfg(test)]
    #[test]
    fn count_dups_correct_for_no_duplicate_letters () {
        assert_eq!(count_dups("abcdef"), (0, 0))
    }

    #[test]
    fn count_dups_correct_for_two_duplicate_letters () {
        assert_eq!(count_dups("abbcde"), (1, 0))
    }

    #[test]
    fn count_dups_correct_for_three_duplicate_letters () {
        assert_eq!(count_dups("abbbcde"), (0, 1))
    }

    #[test]
    fn count_dups_correct_for_four_duplicate_letters () {
        assert_eq!(count_dups("abbbbcde"), (0, 0))
    }

    #[test]
    fn count_dups_correct_for_twos_and_threes () {
        assert_eq!(count_dups("abbcccde"), (1, 1))
    }

    #[test]
    fn find_checksum_correct_for_small_sample () {
        assert_eq!(find_checksum("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12)
    }

    #[test]
    fn compare_ids_returns_true_if_one_character_different () {
        assert_eq!(compare_ids("abcxefg", "abcdefg"), true)
    }

    #[test]
    fn compare_ids_returns_false_if_two_characters_different () {
        assert_eq!(compare_ids("abxxefg", "abcdefg"), false)
    }

    #[test]
    fn compare_ids_returns_false_if_no_characters_different () {
        assert_eq!(compare_ids("abcdefg", "abcdefg"), false)
    }

    #[test]
    fn find_similar_ids_finds_two_ids_with_one_letter_different () {
        assert_eq!(find_similar_ids("abcxefg", &["abcdefg"]), Some("abcdefg"))
    }

    #[test]
    fn find_similar_ids_does_not_find_ids_with_two_letters_different () {
        assert_eq!(find_similar_ids("abxxefg", &["abcdefg"]), None)
    }

    #[test]
    fn find_common_letters_correct_for_sample_input () {
        assert_eq!(find_common_letters("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), Some("fgij".to_string()))
    }

