use std::collections::HashSet;

pub fn has_dups (id: &str, n_dups: usize) -> bool {
    let mut unique_chars = HashSet::new();

    for c in id.chars() {
        let unique = unique_chars.insert(c);
        if !unique {
          return id.matches(c).count() == n_dups;
        }
    }
    false
}

// #[aoc(day2, part1)]
// pub fn find_ (ids: &str) -> bool {
//     for id in ids.lines() {
//         let mut twos: Vec<char> = vec![];
//         let mut unique_chars = HashSet::new();
//         let unique =
//     }
// }

#[cfg(test)]
    #[test]
    fn has_dups_returns_true_for_two_duplicate_letters () {
        assert_eq!(has_dups("abbcde", 2), true)
    }

    #[test]
    fn has_dups_returns_false_for_three_duplicate_letters () {
        assert_eq!(has_dups("abbbcde", 2), false)
    }

    #[test]
    fn has_dups_returns_true_for_three_duplicate_letters () {
        assert_eq!(has_dups("abbbcde", 3), true)
    }

    #[test]
    fn has_dups_returns_false_for_four_duplicate_letters () {
        assert_eq!(has_dups("abbbbcde", 3), false)
    }
