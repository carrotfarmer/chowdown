use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, newline, space0};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::{many0, many1};
use nom::sequence::{terminated, preceded};
use nom::IResult;

use super::Token;


/// Tokenizes the raw string input. A Nom parser.
pub fn parse(markdown: String) -> Vec<Token> {
    
    // Produces a vector of Tokens
    let (residual, tokens): (&str, Vec<_>) = many0(
        alt((
            preceded(space0, alt((
                headings,
                divider,
                blockquote,
                bold,
                italics,
                inline_code,
            ))),
            text,
            endline,
        ))
    )(markdown.as_str())
    .unwrap();

    if residual.len() != 0 {
        println!("residual:\n{residual}");
        panic!();
    }

    tokens
}

fn headings(input: &str) -> IResult<&str, Token> {
    map_res(
        terminated( 
            many1(char::<&str, _>('#')),
            space0
        ),
        |hashtags| -> Result<Token, Error<&str>> {
            let level = hashtags.len();
            Ok(Token::Heading { level })
        },
    )(input)
}

fn divider(input: &str) -> IResult<&str, Token> {
    map_res(tag("---"), |_| -> Result<Token, Error<&str>> {
        Ok(Token::Divider)
    })(input)
}

fn blockquote(input: &str) -> IResult<&str, Token> {
    map_res(
        char('>'),
        |_| -> Result<Token, Error<&str>> {
            Ok(Token::Blockquote)
        }
    )(input)
}

fn bold(input: &str) -> IResult<&str, Token> {
    map_res(
        tag("**"),
        |_| -> Result<Token, Error<&str>> {
            Ok(Token::Bold)
        },
    )(input)
}

fn italics(input: &str) -> IResult<&str, Token> {
    map_res(
        char('*'),
        |_| -> Result<Token, Error<&str>> {
            Ok(Token::Italics)
        },
    )(input)
}

fn inline_code(input: &str) -> IResult<&str, Token> {
    map_res(
        char('`'),
        |_| -> Result<Token, Error<&str>> {
            Ok(Token::InlineCode)
        },
    )(input)
}

fn text(input: &str) -> IResult<&str, Token> {
    map_res( 
        is_not("*``\n\r"),
        |text: &str| -> Result<Token, Error<&str>> {
            Ok(Token::Text{ text: text.to_owned() })
        }
    )(input)
}

fn endline(input: &str) -> IResult<&str, Token> {
    map_res( 
        newline,
        |_| -> Result<Token, Error<&str>> {
            Ok(Token::EndLine)
        }
    )(input)
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
";

        let tokens = parse(input.to_owned());

        let expected = vec![
            Token::Heading {
                level: 1,
            },
            Token::Text {
                text: "Big heading".to_owned(),
            },
            Token::EndLine,
            Token::Text {
                text: "Hello!".to_owned(),
            },
            Token::EndLine,
            Token::Heading {
                level: 3,
            },
            Token::Text {
                text: "Small heading".to_owned(),
            },
            Token::EndLine,
            Token::Text {
                text: "divider?".to_owned(),
            },
            Token::EndLine,
            Token::Divider,
            Token::EndLine,
            Token::Text {
                text: "divided.".to_owned(),
            },
            Token::EndLine,
            Token::Blockquote,
            Token::Text {
                text: "blockquote".to_owned(),
            },
            Token::EndLine,
            Token::Blockquote,
            Token::Text {
                text: "deez".to_owned(),
            },
            Token::EndLine,
            Token::Blockquote,
            Token::Text {
                text: "nuts".to_owned(),
            },
            Token::EndLine,
            Token::Bold,
            Token::Text {
                text: "bold text".to_owned(),
            },
            Token::Bold,
            Token::EndLine,
            Token::Italics,
            Token::Text {
                text: "italic text".to_owned(),
            },
            Token::Italics,
            Token::EndLine,
            Token::Text {
                text: "unclosed ".to_owned(),
            },
            Token::Italics,
            Token::Text {
                text: "italic".to_owned(),
            },
            Token::EndLine,
            Token::InlineCode,
            Token::Text {
                text: "let x = 69;".to_owned(),
            },
            Token::InlineCode,
            Token::EndLine,
        ];

        for (i, tok) in tokens.iter().enumerate() {
            println!("{:?}\t{:?}", tok, expected[i]);
            assert_eq!(*tok, expected[i]);
        }
    }
}
