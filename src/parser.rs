use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, line_ending, multispace0, not_line_ending};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

//Logical representation of markdown elements.
#[derive(Debug)]
pub enum Element {
    Heading { level: u32, text: String },
    Divider,
    Blockquote { text: String },
    Bold { text: String },
    Italics { text: String },
    PlainText { text: String },
}

pub struct Parser {
    contents: String,
}

impl Parser {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    pub fn parse(&self) -> Vec<Element> {
        // Produces a vector of elements
        let (residual, elements): (&str, Vec<_>) = many0(
            /* Wrapped in "multispace0" to remove newlines & spaces */
            alt((
                delimited(
                    multispace0,
                    alt((
                        Self::headings,
                        Self::divider,
                        Self::blockquote,
                        Self::bold,
                        Self::italics,
                        /* plaintext, but mapped into an element */
                        map_res(
                            Self::plain_text,
                            |text: &str| -> Result<Element, Error<&str>> {
                                Ok(Element::PlainText {
                                    text: text.to_owned(),
                                })
                            },
                        ),
                    )),
                    multispace0,
                ),
                /* hacky way to consume all residual text
                 * Handles the "a*b" case.
                 */
                map_res(
                    is_not("\n\r"),
                    |text: &str| -> Result<Element, Error<&str>> {
                        Ok(Element::PlainText {
                            text: text.to_owned(),
                        })
                    },
                ),
            )),
        )(self.contents.as_str())
        .unwrap();

        if residual.len() != 0 {
            println!("should not be any residual.");
            println!("residual:\n{residual}");
            panic!();
        }

        return elements;
    }

    fn headings(input: &str) -> IResult<&str, Element> {
        map_res(
            terminated(
                pair(many1(char::<&str, _>('#')), not_line_ending),
                line_ending,
            ),
            |(hashtags, text)| -> Result<Element, Error<&str>> {
                // Heading "level" (size) is defined by the amount of '#'s
                let level = hashtags.len() as u32;
                Ok(Element::Heading {
                    level,
                    text: text.trim().to_owned(),
                })
            },
        )(input)
    }

    fn divider(input: &str) -> IResult<&str, Element> {
        map_res(tag("---\n"), |_| -> Result<Element, Error<&str>> {
            Ok(Element::Divider)
        })(input)
    }

    fn blockquote(input: &str) -> IResult<&str, Element> {
        map_res(
            many1(delimited(char('>'), not_line_ending, line_ending)),
            |texts| -> Result<Element, Error<&str>> {
                let text = texts.into_iter().fold(String::new(), |mut a, line: &str| {
                    a.push_str(line.trim());
                    a.push_str("\r\n");
                    a
                });

                Ok(Element::Blockquote { text })
            },
        )(input)
    }

    fn bold(input: &str) -> IResult<&str, Element> {
        map_res(
            delimited(tag("**"), Self::plain_text, tag("**")),
            |text: &str| -> Result<Element, Error<&str>> {
                Ok(Element::Bold {
                    text: text.to_owned(),
                })
            },
        )(input)
    }

    fn italics(input: &str) -> IResult<&str, Element> {
        map_res(
            delimited(tag("*"), Self::plain_text, tag("*")),
            |text: &str| -> Result<Element, Error<&str>> {
                Ok(Element::Italics {
                    text: text.to_owned(),
                })
            },
        )(input)
    }

    fn plain_text(input: &str) -> IResult<&str, &str> {
        is_not("*\n\r")(input)
    }
}