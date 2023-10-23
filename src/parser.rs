use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, line_ending, multispace0, not_line_ending};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

//Logical representation of markdown elements.
#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    Heading { level: usize, text: String },
    Divider,
    Blockquote { text: String },
    Bold { text: String },
    Italics { text: String },
    Strikethrough { text: String },
    PlainText { text: String },
    InlineCode { text: String },
}

pub struct Parser {
    contents: String,
}

impl Parser {
    pub fn new(contents: String) -> Self {
        let mut new_contents = contents.clone();

        if !new_contents.ends_with("\n") {
            println!("contents does not end with \\n");
            new_contents.push_str("\n");
        }

        Self {
            contents: new_contents,
        }
    }

    pub fn parse(&self) -> Vec<Element> {
        // Produces a vector of elements
        let (_residual, elements): (&str, Vec<_>) = many0(
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
                        Self::inline_code,
                        Self::strikethrough,
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
                let level = hashtags.len();
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

    fn inline_code(input: &str) -> IResult<&str, Element> {
        map_res(
            delimited(tag("`"), Self::plain_text, tag("`")),
            |text: &str| -> Result<Element, Error<&str>> {
                Ok(Element::InlineCode {
                    text: text.to_owned(),
                })
            },
        )(input)
    }

    fn strikethrough(input: &str) -> IResult<&str, Element> {
        map_res(
            delimited(tag("~~"), Self::plain_text, tag("~~")),
            |text: &str| -> Result<Element, Error<&str>> {
                Ok(Element::Strikethrough {
                    text: text.to_owned(),
                })
            },
        )(input)
    }

    fn plain_text(input: &str) -> IResult<&str, &str> {
        is_not("*`~\n\r")(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown() {
        let input = r"
# Big heading
Hello!
### Small heading
divider?
---
divided.
> blockquote
> deez
> nuts
**bold text**
*italic text*
unclosed *italic
`let x = 69;`
~~strikethru text~~
";

        let elements = Parser::new(input.to_owned()).parse();

        let expected = vec![
            Element::Heading {
                level: 1,
                text: "Big heading".to_owned(),
            },
            Element::PlainText {
                text: "Hello!".to_owned(),
            },
            Element::Heading {
                level: 3,
                text: "Small heading".to_owned(),
            },
            Element::PlainText {
                text: "divider?".to_owned(),
            },
            Element::Divider,
            Element::PlainText {
                text: "divided.".to_owned(),
            },
            Element::Blockquote {
                text: "blockquote\r\ndeez\r\nnuts\r\n".to_owned(),
            },
            Element::Bold {
                text: "bold text".to_owned(),
            },
            Element::Italics {
                text: "italic text".to_owned(),
            },
            Element::PlainText {
                text: "unclosed ".to_owned(),
            },
            Element::PlainText {
                text: "*italic".to_owned(),
            },
            Element::InlineCode {
                text: "let x = 69;".to_owned(),
            },
            Element::Strikethrough {
                text: "strikethru text".to_owned(),
            },
        ];

        assert_eq!(elements, expected);
    }
}
