use std::num::ParseIntError;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn includes(&self, loc: (usize, usize)) -> bool {
        let (left, top) = loc;
        self.left <= left
            && self.left + self.width - 1 >= left
            && self.top <= top
            && self.top + self.height - 1 >= top
    }
}

#[aoc_generator(day3)]
pub fn parse_claims (input: &str) -> Result<Vec<Claim>, ParseIntError> {
    let re = Regex::new(r"#([\d]+) @ ([\d]+),([\d]+): ([\d]+)x([\d]+)").unwrap();
    let mut claims: Vec<Claim> = vec![];

    for cap in re.captures_iter(input) {
        claims.push(Claim {
            id: cap[1].parse()?,
            left: cap[2].parse()?,
            top: cap[3].parse()?,
            width: cap[4].parse()?,
            height: cap[5].parse()?,
        });
    }
    Ok(claims)
}

pub fn sheet_dimensions (claims: &[Claim]) -> (usize, usize) {
    let mut right = 0;
    let mut bottom = 0;

    for claim in claims.iter() {
        let r = claim.left + claim.width;
        if r > right {
            right = r;
        }
        let b = claim.top + claim.height;
        if b > bottom {
            bottom = b;
        }
    }
    (right as usize, bottom as usize)
}

pub fn has_conflict (loc: (usize, usize), claims: &[Claim]) -> bool {
    let mut overlaps = 0;

    for claim in claims.iter() {
        if claim.includes(loc) {
            overlaps += 1;
        }

        if overlaps == 2 {
            return true;
        }
    }
    false
}

#[aoc(day3, part1)]
pub fn conflict_area (claims: &[Claim]) -> u32 {
    let (sheet_width, sheet_height) = sheet_dimensions(claims);
    let mut sheet: Vec<Vec<bool>> = vec![];

    for top in 0..sheet_height {
        let mut conflicts: Vec<bool> = vec![];
        for left in 0..sheet_width {
            conflicts.push(has_conflict((left, top), claims));
        }
        sheet.push(conflicts);
    }

    sheet
        .iter()
        .fold(0, |inches, row| inches + row
            .iter()
            .fold(0, |sum, col| sum + *col as u32))
}


#[cfg(test)]

    #[test]
    fn parse_claims_returns_vec_of_claims () {
        assert_eq!(
            Ok(vec![
                Claim{ id: 1, left: 829, top: 837, width: 11, height: 22, },
                Claim{ id: 2, left: 14, top: 171, width: 10, height: 16, },
            ]),
            parse_claims("#1 @ 829,837: 11x22\n#2 @ 14,171: 10x16\n")
        )
    }

    #[test]
    fn sheet_dimensions_finds_largest_right_and_bottom_positions () {
        assert_eq!(
            sheet_dimensions(&vec![
                Claim{ id: 1, left: 89, top: 10, width: 10, height: 33 },
                Claim{ id: 2, left: 13, top: 91, width: 20, height: 10 },
                Claim{ id: 3, left: 84, top: 85, width: 15, height: 5 },
            ]),
            (99, 101)
        )
    }

    #[test]
    fn has_conflict_returns_false_for_1_overlap () {
        let claims = &vec![
            // #1 @ 1,3: 4x4
            // #2 @ 3,1: 4x4
            // #3 @ 5,5: 2x2
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(has_conflict((3, 1), claims), false)
    }

    #[test]
    fn has_conflict_returns_true_for_2_overlaps () {
        let claims = &vec![
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(has_conflict((3, 3), claims), true)
    }

    #[test]
    fn has_conflict_returns_false_for_0_overlaps () {
        let claims = &vec![
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(has_conflict((1, 1), claims), false)
    }

    #[test]
    fn has_conflict_returns_true_for_greater_than_2_overlaps () {
        let claims = &vec![
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
            Claim{ id: 4, left: 1, top: 2, width: 4, height: 5 },
        ];
        assert_eq!(has_conflict((3, 3), claims), true)
    }

    #[test]
    fn claim_includes_true_if_loc_within_bounds () {
        let claim = Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 };
        assert_eq!(claim.includes((1, 3)), true)
    }

    #[test]
    fn claim_includes_false_if_top_out_of_bounds () {
        let claim = Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 };
        assert_eq!(claim.includes((1, 8)), false)
    }

    #[test]
    fn claim_includes_false_if_left_out_of_bounds () {
        let claim = Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 };
        assert_eq!(claim.includes((6, 3)), false)
    }

    #[test]
    fn conflict_area_correct_for_small_sample () {
        let claims = &vec![
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
        ];
        assert_eq!(conflict_area(claims), 5)
    }

