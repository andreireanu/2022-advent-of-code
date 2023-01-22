// crates parsed with nom library

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    sequence::delimited,
    *,
};

fn parse_crate(input: &str) -> IResult<&str, &str> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;
    let out = match c {
        "   " => "_",
        value => value,
    };
    Ok((input, out))
}

fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, c) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, c))
}

pub fn part_one(s: &str) {
    let mut lines = s.lines().collect::<Vec<&str>>().into_iter();
    while let Some(line) = lines.next() {
        let result = parse_line(line);
        match result {
            Ok((_left_string, vec)) => {
                println!("{:?}", vec);
            }
            Err(_) => {}
        };
    }
}
