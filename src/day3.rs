use std::collections::HashSet;
use std::num::ParseIntError;
use regex::Regex;

#[derive(Eq, Debug, Hash, PartialEq)]
pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Claim {
    fn includes(&self, loc: (u32, u32)) -> bool {
        let (left, top) = loc;
        let right = self.left + self.width - 1;
        let bottom = self.top + self.height - 1;

        self.left <= left
            && right >= left
            && self.top <= top
            && bottom >= top
    }

    fn has_conflict_with(&self, other: &Claim) -> bool {
        let right = self.left + self.width;
        let bottom = self.top + self.height;

        for y in self.top..bottom {
            for x in self.left..right {
                if other.includes((x, y)) {
                    return true;
                }
            }
        }
        false
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

#[aoc(day3, part1)]
pub fn conflict_area (claims: &[Claim]) -> u32 {
    let mut points = HashSet::<(u32, u32)>::new();
    let mut conflicts = HashSet::<(u32, u32)>::new();

    for claim in claims.iter() {
        let right = claim.left + claim.width;
        let bottom = claim.top + claim.height;

        for y in claim.top..bottom {
            for x in claim.left..right {
                let available = points.insert((x, y));
                if !available {
                    conflicts.insert((x, y));
                }
            }
        }
    }
    conflicts.iter().count() as u32
}

#[aoc(day3, part2)]
pub fn best_claim (claims: &[Claim]) -> u32 {
    let mut conflicts: HashSet<&Claim> = HashSet::new();

    for claim in claims.iter() {
        if conflicts.contains(claim) {
            continue;
        }

        for compare in claims.iter() {
            if claim.id == compare.id {
                continue;
            }
            if claim.has_conflict_with(compare) {
                conflicts.insert(claim);
                conflicts.insert(compare);
            }
        }
    }

    let best = claims.iter().find(|claim| !conflicts.contains(claim)).unwrap();
    best.id
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
        assert_eq!(conflict_area(claims), 4)
    }

    #[test]
    fn best_claim_identifies_the_claim_without_conflicts () {
        let claims = &vec![
            Claim{ id: 1, left: 1, top: 3, width: 4, height: 4 },
            Claim{ id: 2, left: 3, top: 1, width: 4, height: 4 },
            Claim{ id: 3, left: 5, top: 5, width: 2, height: 2 },
            Claim{ id: 4, left: 1, top: 4, width: 2, height: 1 },
        ];
        assert_eq!(best_claim(claims), 3)
    }

