use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::{pair, terminated};
use nom::IResult;

//Logical representation of markdown elements.
#[derive(Debug)]
enum Element {
    Heading { level: u32, text: String },
    Divider,
    PlainText { text: String },
    Blockquote { text: String },
}

fn headings(input: &str) -> IResult<&str, Element> {
    // Add a function to convert -- map -- the parsed string into our Enum.
    map_res(
        /* Bundles two parsers together, and outputs (A, B),
         * the outputs of the two parsers respectively.
         */
        pair(
            many1(is_a::<&str, &str, _>("#")),
            not_line_ending::<&str, _>,
        ),
        |(hashtags, text)| -> Result<Element, nom::error::Error<&str>> {
            // Heading "level" (size) is defined by the amount of '#'s
            let level = hashtags.len() as u32;
            Ok(Element::Heading {
                level,
                text: text.to_owned(),
            })
        },
    )(input)
}

fn divider(input: &str) -> IResult<&str, Element> {
    map_res(
        tag("---"),
        |_| -> Result<Element, nom::error::Error<&str>> { Ok(Element::Divider) },
    )(input)
}

fn blockquote(input: &str) -> IResult<&str, Element> {
    map_res(
        /* Bundles two parsers together, and outputs (A, B),
         * the outputs of the two parsers respectively.
         */
        pair(is_a::<&str, &str, _>("> "), not_line_ending::<&str, _>),
        |(_, text)| -> Result<Element, nom::error::Error<&str>> {
            // Heading "level" (size) is defined by the amount of '#'s
            Ok(Element::Blockquote {
                text: text.to_owned(),
            })
        },
    )(input)
}

fn plain_text(input: &str) -> IResult<&str, Element> {
    map_res(
        not_line_ending::<&str, _>,
        |text| -> Result<Element, nom::error::Error<&str>> {
            Ok(Element::PlainText {
                text: text.to_owned(),
            })
        },
    )(input)
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
> blockquote deez nuts
";

    // Produces a vector of elements
    let (residual, elements): (&str, Vec<_>) = many0(
        /* Must end with a "line ending" (/r or /n) character.
         * We use 'terminated()' here because it does not return the terminator,
         * only the result of the first parser.
         */
        terminated(
            alt((headings, divider, blockquote, plain_text)),
            line_ending,
        ),
    )(string)
    .unwrap();

    if residual.len() != 0 {
        println!("should not be any residual.");
        println!("residual:\n{residual}");
        panic!();
    }

    /* Expects:
     * Element::Heading{ level: 1, text: "Big heading" }
     * Element::PlainText{ text: "Hello!" }
     * Element::Heading{ level: 3, text: "Small heading" }
     * Element::PlainText{ text: "divider?" }
     * Element::Divider
     * Element::PlainText{ text: "divided." }
     */
    elements.iter().for_each(|e| println!("{:?}", e));
}
