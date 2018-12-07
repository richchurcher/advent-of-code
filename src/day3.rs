use std::num::ParseIntError;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

#[aoc_generator(day3)]
pub fn parse_claims (input: &str) -> Result<Vec<Claim>, ParseIntError> {
    let re = Regex::new(r"#([\d]+) @ ([\d]+),([\d]+): ([\d]+)x([\d]+)\n").unwrap();
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

// #[aoc(day3, part1)]


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
