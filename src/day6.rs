#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
    area: u32,
}

impl Point {
    // top, bottom, left, right
    fn is_infinite (&self, bounding_box: (i32, i32, i32, i32)) -> bool {
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
                    area: 0,
                }
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn find_largest_area (input: &[Point]) -> u32 {
    let mut coordinates = input.to_owned();
    let bounding_box = find_bounding_box(&coordinates);

    for y in bounding_box.0 + 1..=bounding_box.1 - 1 {
        for x in bounding_box.2 + 1..=bounding_box.3 - 1 {
            match find_closest_index(&input, x, y) {
                Some(closest_index) => coordinates[closest_index].area += 1,
                None => continue,
            }
        }
    }

    coordinates
        .iter()
        .filter(|point| !point.is_infinite(bounding_box))
        .max_by_key(|point| point.area)
        .unwrap()
        .area
}

fn find_closest_index (coordinates: &[Point], x: i32, y: i32) -> Option<usize> {
    let mut distances = vec![];
    for point in coordinates {
        distances.push(find_distance(&point, x, y));
    }
    let closest_index = distances
        .iter()
        .enumerate()
        .min_by_key(|&(_, n)| n)
        .unwrap()
        .0;
    distances.sort();
    if distances[0] == 0 || distances[0] != distances[1] {
        return Some(closest_index);
    }
    None
}

fn find_distance (to: &Point, x: i32, y: i32) -> i32 {
    (y - to.y).abs() + (x - to.x).abs()
}

fn find_bounding_box (points: &[Point]) -> (i32, i32, i32, i32) {
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
                Point { x: 1, y: 1, area: 0, },
                Point { x: 1, y: 6, area: 0, },
                Point { x: 8, y: 3, area: 0, },
                Point { x: 3, y: 4, area: 0, },
                Point { x: 5, y: 5, area: 0, },
                Point { x: 8, y: 9, area: 0, },
            ]
        )
    }

    #[test]
    fn find_bounding_box_correct_for_sample_data () {
        assert_eq!(
            find_bounding_box(
                &[
                    Point { x: 1, y: 1, area: 0, },
                    Point { x: 1, y: 6, area: 0, },
                    Point { x: 8, y: 3, area: 0, },
                    Point { x: 3, y: 4, area: 0, },
                    Point { x: 5, y: 5, area: 0, },
                    Point { x: 8, y: 9, area: 0, },
                ]),
                (1, 9, 1, 8)
            )
    }

    #[test]
    fn is_infinite_true_if_at_top_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 1, area: 0, }.is_infinite((1, 5, 0, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_bottom_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 5, area: 0, }.is_infinite((1, 5, 0, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_left_of_bounding_box () {
        assert_eq!(Point { x: 1, y: 1, area: 0, }.is_infinite((0, 5, 1, 5)), true)
    }

    #[test]
    fn is_infinite_true_if_at_right_of_bounding_box () {
        assert_eq!(Point { x: 5, y: 1, area: 0, }.is_infinite((0, 5, 1, 5)), true)
    }

    #[test]
    fn is_infinite_false_if_inside_bounding_box () {
        assert_eq!(Point { x: 2, y: 2, area: 0, }.is_infinite((1, 3, 1, 3)), false)
    }

    #[test]
    fn find_distance_correct_for_horizontally_adjacent () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 0, 1), 1)
    }

    #[test]
    fn find_distance_correct_for_horizontally_distant () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 5, 1), 4)
    }

    #[test]
    fn find_distance_correct_for_vertically_adjacent () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 1, 0), 1)
    }

    #[test]
    fn find_distance_correct_for_vertically_distant () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 1, 5), 4)
    }

    #[test]
    fn find_distance_correct_for_diagonally_adjacent () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 0, 0), 2)
    }

    #[test]
    fn find_distance_correct_for_diagonally_distant () {
        assert_eq!(find_distance(&Point { x: 1, y: 1, area: 0, }, 5, 5), 8)
    }

    #[test]
    fn find_closest_index_identifies_self_as_closest () {
        assert_eq!(
            find_closest_index(
                &vec![
                    Point { x: 1, y: 1, area: 0, },
                    Point { x: 1, y: 6, area: 0, },
                    Point { x: 8, y: 3, area: 0, },
                    Point { x: 3, y: 4, area: 0, },
                    Point { x: 5, y: 5, area: 0, },
                    Point { x: 8, y: 9, area: 0, },
                ],
                5,
                5
            ),
            Some(4)
        )
    }

    #[test]
    fn find_closest_index_correct_at_boundary () {
        assert_eq!(
            find_closest_index(
                &vec![
                    Point { x: 1, y: 1, area: 0, },
                    Point { x: 1, y: 6, area: 0, },
                    Point { x: 8, y: 3, area: 0, },
                    Point { x: 3, y: 4, area: 0, },
                    Point { x: 5, y: 5, area: 0, },
                    Point { x: 8, y: 9, area: 0, },
                ],
                4,
                1
            ),
            Some(0)
        )
    }
    #[test]
    fn find_largest_area_correct_for_sample_data () {
        assert_eq!(
            find_largest_area(
                &vec![
                    Point { x: 1, y: 1, area: 0, },
                    Point { x: 1, y: 6, area: 0, },
                    Point { x: 8, y: 3, area: 0, },
                    Point { x: 3, y: 4, area: 0, },
                    Point { x: 5, y: 5, area: 0, },
                    Point { x: 8, y: 9, area: 0, },
                ]
            ),
            17
        )
    }

