use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, not_line_ending, char, multispace0};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

//Logical representation of markdown elements.
#[derive(Debug)]
enum Element {
    Heading { level: u32, text: String },
    Divider,
    Blockquote { text: String },
    Bold { text: String  },
    Italics { text: String },
    PlainText { text: String },
}

fn headings(input: &str) -> IResult<&str, Element> {
    map_res(
        terminated(
            pair(
                many1(char::<&str, _>('#')),
                not_line_ending,
            ),
            line_ending
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
        many1(
            delimited(char('>'), not_line_ending, line_ending),
        ),
        |texts| -> Result<Element, Error<&str>> {
            let text = texts
                .into_iter()
                .fold(String::new(), |mut a, line: &str| {
                    a.push_str(line.trim());
                    a.push_str("\r\n");
                    a
                });

            Ok(Element::Blockquote {
                text
            })
        },
    )(input)
}

fn bold(input: &str) -> IResult<&str, Element> {
    map_res(
        delimited(tag("**"), plain_text, tag("**")),
        |text: &str| -> Result<Element, Error<&str>> {
            Ok(Element::Bold {
                text: text.to_owned()
            })
        },
    )(input)
}

fn italics(input: &str) -> IResult<&str, Element> {
    map_res(
        delimited(tag("*"), plain_text, tag("*")),
        |text: &str| -> Result<Element, Error<&str>> {
            Ok(Element::Italics {
                text: text.to_owned()
            })
        },
    )(input)
}

fn plain_text(input: &str) -> IResult<&str, &str> {
    is_not("*\n\r")(input)
}

fn main() {
    /* get input */
    let string = r"
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
hello **bold** world
unclosed *italic
    ".trim();


    // Produces a vector of elements
    let (residual, elements): (&str, Vec<_>) = many0(
        /* Wrapped in "multispace0" to remove newlines & spaces */
        alt((
            delimited(
                multispace0,
                alt((
                    headings,
                    divider,
                    blockquote,
                    bold,
                    italics,

                    /* plaintext, but mapped into an element */
                    map_res(
                        plain_text,
                        |text: &str| -> Result<Element, Error<&str>> {
                            Ok(Element::PlainText {
                                text: text.to_owned(),
                            })
                        },
                    ),

                           )),
                multispace0
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
            )
        ))
    )(string)
        .unwrap();

    if residual.len() != 0 {
        println!("should not be any residual.");
        println!("residual:\n{residual}");
        panic!();
    }

    // Print Result
    elements
        .iter()
        .for_each(|e| println!("{:?}", e));
}
