#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    // top, bottom, left, right
    fn is_infinite (&self, bounding_box: (u32, u32, u32, u32)) -> bool {
        self.y == bounding_box.0
            || self.y == bounding_box.1
            || self.x == bounding_box.2
            || self.x == bounding_box.3
    }
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

#[aoc(day6, part1)]
fn solve (input: &[Point]) -> u32 {
    0
}

fn find_bounding_box (points: &[&Point]) -> (u32, u32, u32, u32) {
    let mut top = points[0].y;
    let mut bottom = points[0].y;
    let mut left = points[0].x;
    let mut right = points[0].x;

    for point in points {
        if point.y < top {
            top = point.y;
        }
        if point.y > bottom {
            bottom = point.y;
        }
        if point.x < left {
            left = point.x;
        }
        if point.x > right {
            right = point.x;
        }
    }
    (top, bottom, left, right)
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

    #[test]
    fn find_bounding_box_correct_for_sample_data () {
        assert_eq!(
            find_bounding_box(
                &[
                    &Point { x: 1, y: 1, },
                    &Point { x: 1, y: 6, },
                    &Point { x: 8, y: 3, },
                    &Point { x: 3, y: 4, },
                    &Point { x: 5, y: 5, },
                    &Point { x: 8, y: 9, },
                ]),
                (1, 9, 1, 8)
            )
    }

    #[test]
    fn is_infinite_true_if_at_top_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 1 }.is_infinite((1, 5, 0, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_bottom_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 5 }.is_infinite((1, 5, 0, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_left_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 1 }.is_infinite((0, 5, 1, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_right_of_bounding_box () {
        assert_eq!(Point { x: 5, y: 1 }.is_infinite((0, 5, 1, 5)), true)
    }

    #[test]
    fn is_infinite_false_if_inside_bounding_box () {
        assert_eq!(Point { x: 2, y: 2 }.is_infinite((1, 3, 1, 3)), false)
    }

