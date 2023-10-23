use std::collections::HashMap;

use displaydoc::Display;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag},
    character::complete::one_of,
    combinator::{eof, map},
    multi::fold_many0,
    sequence::{delimited, separated_pair},
    IResult,
};

fn parse_setting(input: &str) -> IResult<&str, (String, String)> {
    map(
        separated_pair(
            is_not::<_, &str, _>("="),
            tag("="),
            alt((
                delimited(
                    tag("\""),
                    escaped(is_not("\""), '\\', one_of("\"\\")),
                    tag("\""),
                ),
                is_not(";"),
            )),
        ),
        |(x, y)| (x.to_string(), y.to_string()),
    )(input)
}

//pub type StringMap = Vec<(String, String)>;
pub type StringMap = HashMap<String, String>;

fn parse_connection_setting(input: &str) -> IResult<&str, (String, String)> {
    let (input, set) = parse_setting(input)?;
    let (input, _) = alt((tag(";"), eof))(input)?;
    Ok((input, set))
}

fn parse_connection_string(input: &str) -> IResult<&str, StringMap> {
    fold_many0(
        parse_connection_setting,
        HashMap::new,
        |mut acc: StringMap, (key, value): (String, String)| {
            acc.insert(key, value);
            acc
        },
    )(input)
}

#[derive(Debug, Display)]
/// Failed to load settings from connection string
pub struct SettingsError;
impl std::error::Error for SettingsError {}

pub fn get_settings(val: String) -> Result<StringMap, SettingsError> {
    parse_connection_string(val.as_str())
        .map(|y| y.1)
        .map_err(|_| SettingsError)
}
