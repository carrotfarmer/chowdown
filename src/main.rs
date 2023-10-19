use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, line_ending, multispace0, not_line_ending};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

mod parser;

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
    "
    .trim();

    let elements = parser::Parser::new(string.to_owned()).parse();

    // Print Result
    elements.iter().for_each(|e| println!("{:?}", e));
}
