#[aoc(day1, part1)]
pub fn calibrate_device (input: &str) {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_frequency_for_three_values () {
        assert_eq(2, calibrate_device("1, -1, 2"));
    }
}

