use nom::{
    IResult, Parser,
    bytes::complete::{escaped, tag, take_till1},
    character::complete::{char, none_of, space1},
    multi::many1,
    sequence::delimited,
};

pub enum Op {
    GET(String),
    SET(String, String),
    HSET(String, Vec<(String, String)>),
    HGET(String, String),
}

fn parse_simple_string(input: &str) -> IResult<&str, String> {
    let (input, s) = take_till1(|s| s == ' ')(input)?;
    Ok((input, s.to_string()))
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    let (input, s) = delimited(
        char('"'),
        escaped(none_of("\\\""), '\\', char('"')),
        char('"'),
    )
    .parse(input)?;
    Ok((input, s.to_string()))
}

pub fn parse_str_set(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("SET")(input)?;
    let (input, _) = space1(input)?;
    let (input, k) = parse_simple_string(input)?;
    let (input, _) = space1(input)?;
    let (input, v) = parse_quoted_string(input)?;
    Ok(("", Op::SET(k.to_string(), v.to_string())))
}

pub fn parse_hash_set(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("HSET")(input)?;
    let (input, _) = space1(input)?;
    let (input, k) = parse_simple_string(input)?;
    loop {
        let (input, _) = space1(input)?;
        let (input, ink) = parse_simple_string(input)?;
        let (input, inv) = parse_quoted_string(input)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = parse_hash_set("HSET supk k1 \"v1\"").unwrap().0;
        assert_eq!(result, "");
    }
}
