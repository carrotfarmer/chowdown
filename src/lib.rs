mod lexer;
mod parser;
mod generator;

use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


#[cfg(target_family = "wasm")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


//Logical representation of markdown elements.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    NumList { num: u8, indent: u8 },
    List { indent: u8 },
    Heading { level: usize },
    Divider,
    Blockquote,
    Bold,
    Italics,
    InlineCode,
    Text { text: String },
    EndLine,
}


#[derive(Debug, Clone)]
pub enum Element {
    Dummy,
    List { elements: Vec<Element> },
    NumList { elements: Vec<Element> },
    Heading { level: usize, text: String },
    Divider,
    Blockquote { text: String },
    Bold { text: String },
    Italics { text: String },
    PlainText { text: String },
    InlineCode { text: String },
    Text { elements: Vec<Element> }
}



#[wasm_bindgen]
pub fn parse(markdown: String) -> String {

    #[cfg(target_family = "wasm")]
    console_log!("hello");

    // Tokenizes the raw input
    let elements = lexer::parse(markdown);
    println!("{:?}", elements);

    // Parses the token stream into a vector of elements.
    let tree = parser::parse(elements);
    println!("{:?}", tree);

    // Generate
    let html = generator::generate(tree);

    html
}
