use nom::branch::alt;
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, terminated};
use nom::bytes::complete::take;
use nom::IResult;
use nom::InputTake;
use nom::InputIter;
use nom::InputLength;
use nom::Err;
use nom::error::Error;
use nom::error::ErrorKind;

use std::iter::Enumerate;

use super::{Token, Element};


/// Custom input type for parser 
#[derive(Clone, Debug)]
struct Tokens<'a> {
    v: &'a [Token]
}


/// Required implementation for nom
impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.v.len()
    }
}

/// Required implementation for nom
impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens {
            v: &self.v[0..count],
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let a = Tokens {
            v: &self.v[0..count],
        };
        let b = Tokens {
            v: &self.v[count..],
        };
        (b, a)
    }
}

/// Required implementation for nom
impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token;
    type Iter = Enumerate<::std::slice::Iter<'a, Token>>;
    type IterElem = ::std::slice::Iter<'a, Token>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.v.iter().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.v.iter()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.v.iter().position(predicate)
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.v.len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::Unknown)
        }
    }
}


/// Parses the token stream into elements. Nom parser.
pub fn parse (tokens: Vec<Token>) -> Vec<Element> {

    let tokens = Tokens {
        v: &tokens[..]
    };

    let (_residual, elems): (Tokens, Vec<Element>) = many0(
        alt((
            text,
        ))
    )(tokens)
        .unwrap();

    elems 
}

fn text (i: Tokens) -> IResult<Tokens, Element> {
    map_res(
        terminated(
            many1(alt((
                plain,
                bold,
                italic,
                inline_code,

                unclosed_tags
            ))),
            is_endline
        ),
        |v| -> Result<Element, Err<Tokens>> {
            Ok( Element::Text { elements: v } )
        }
    )(i)
}

fn italic (i: Tokens) -> IResult<Tokens, Element> {
    map_res(
        delimited(is_italic, plain, is_italic),
        |plain| -> Result<Element, Err<Tokens>> {
            let text = if let Element::PlainText { text } = plain { text } else { panic!() };
            Ok( Element::Bold { text } )
        }
    )(i)
}

fn bold (i: Tokens) -> IResult<Tokens, Element> {
    map_res(
        delimited(is_bold, plain, is_bold),
        |plain| -> Result<Element, Err<Tokens>> {
            let text = if let Element::PlainText { text } = plain { text } else { panic!() };
            Ok( Element::Bold { text } )
        }
    )(i)
}

fn inline_code (i: Tokens) -> IResult<Tokens, Element> {
    map_res(
        delimited(is_inline_code, plain, is_inline_code),
        |plain| -> Result<Element, Err<Tokens>> {
            let text = if let Element::PlainText { text } = plain { text } else { panic!() };
            Ok( Element::Bold { text } )
        }
    )(i)
}

fn plain (i: Tokens) -> IResult<Tokens, Element> {
    let (i, tokens) = take(1usize)(i)?;
    if tokens.v.is_empty() { 
        return Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
    if let Token::Text { ref text } = tokens.v[0] {
        Ok(( i, Element::PlainText { text: text.clone() } ))
    } else {
        Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
}

fn unclosed_tags (i: Tokens) -> IResult<Tokens, Element> {
    alt((is_bold, is_italic))(i)
}


// === The below functions parses a single token variant, regardless of data ==
// -> returns a "PlainText" element with the textual representation of that element, if there is one.

fn is_endline (i: Tokens) -> IResult<Tokens, Element> {
    let (i, tokens) = take(1usize)(i)?;
    if tokens.v.is_empty() { 
        return Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
    if let Token::EndLine = tokens.v[0] {
        Ok(( i, Element::PlainText { text: "\n".to_owned() } ))
    } else {
        Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
}

fn is_bold (i: Tokens) -> IResult<Tokens, Element> {
    let (i, tokens) = take(1usize)(i)?;
    if tokens.v.is_empty() { 
        return Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
    if let Token::Bold = tokens.v[0] {
        Ok(( i, Element::PlainText { text: "**".to_owned() } ))
    } else {
        Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
}

fn is_italic (i: Tokens) -> IResult<Tokens, Element> {
    let (i, tokens) = take(1usize)(i)?;
    if tokens.v.is_empty() { 
        return Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
    if let Token::Italics = tokens.v[0] {
        Ok(( i, Element::PlainText { text: "*".to_owned() } ))
    } else {
        Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
}

fn is_inline_code (i: Tokens) -> IResult<Tokens, Element> {
    let (i, tokens) = take(1usize)(i)?;
    if tokens.v.is_empty() { 
        return Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
    if let Token::InlineCode = tokens.v[0] {
        Ok(( i, Element::PlainText { text: "`".to_owned() } ))
    } else {
        Err(Err::Error(Error::new(i, ErrorKind::Tag))) 
    }
}


