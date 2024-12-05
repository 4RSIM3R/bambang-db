use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::{alphanumeric1, char, space0, space1},
    combinator::{map, opt, recognize},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Columns<'a> {
    All,
    Some(Vec<&'a str>),
}

#[derive(Debug, PartialEq)]
pub struct Condition<'a> {
    pub column: &'a str,
    pub operator: &'a str,
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct SelectQuery<'a> {
    pub columns: Columns<'a>,
    pub table: &'a str,
    pub condition: Option<Condition<'a>>, // Optional WHERE condition
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((nom::character::complete::alpha1, tag("_"))),
        nom::bytes::complete::take_while(|c: char| c.is_alphanumeric() || c == '_'),
    )))(input)
}

fn columns_parser(input: &str) -> IResult<&str, Columns> {
    alt((
        map(tag("*"), |_| Columns::All),
        map(
            separated_list0(preceded(char(','), space0), identifier),
            Columns::Some,
        ),
    ))(input)
}

fn condition_parser(input: &str) -> IResult<&str, Condition> {
    map(
        tuple((
            identifier,                          // column name
            space1,                              // space after column
            alt((tag("="), tag(">"), tag("<"))), // operator
            space1,                              // space after operator
            alt((alphanumeric1, is_not(" ;"))),  // value (string, number, etc.)
        )),
        |(column, _, operator, _, value): (&str, &str, &str, &str, &str)| Condition {
            column,
            operator,
            value,
        },
    )(input)
}

fn table_parser(input: &str) -> IResult<&str, &str> {
    preceded(tag_no_case("FROM"), preceded(space1, is_not(" "))) (input)
}

pub fn parse_select(input: &str) -> IResult<&str, SelectQuery> {
    map(
        tuple((
            tag_no_case("SELECT"),      // SELECT keyword
            space0,                     // optional spaces
            columns_parser,             // columns (e.g., "*" or "col1, col2")
            space0,                     // optional spaces
            table_parser,               // FROM clause (e.g., "table_name")
            space0,                     // optional spaces
            opt(preceded(               // Optional WHERE clause
                tag_no_case("WHERE"),
                preceded(space1, condition_parser),
            )),
            tag(";"),                   // End with semicolon
        )),
        |(_, _, columns, _, table, _, condition, _)| SelectQuery {
            columns,
            table,
            condition,
        },
    )(input)
}
