#[derive(Debug, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

#[aoc_generator(day6)]
fn parse_points (input: &str) -> Vec<Point> {
    input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(", ").collect();
            Some(
                Point {
                    x: parts[0].parse().ok()?,
                    y: parts[1].parse().ok()?,
                }
            )
        })
        .collect()
}

#[cfg(test)]

    #[test]
    fn parse_points_returns_vec_of_point () {
        assert_eq!(
            parse_points("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9"),
            vec![
                Point { x: 1, y: 1, },
                Point { x: 1, y: 6, },
                Point { x: 8, y: 3, },
                Point { x: 3, y: 4, },
                Point { x: 5, y: 5, },
                Point { x: 8, y: 9, },
            ]
        )
    }

