mod lexer;
mod parser;
mod generator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


//Logical representation of markdown elements.
#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    Heading { level: usize, text: String },
    Divider,
    Blockquote { text: String },
    Bold { text: String },
    Italics { text: String },
    PlainText { text: String },
    InlineCode { text: String },
}


#[derive(Debug)]
pub struct Node {
    element: Element,
    children: Vec<Self>
}


#[wasm_bindgen]
pub fn parse(markdown: String) -> String {

    #[cfg(target_family = "wasm")]
    console_log!("hello");

    let elements = lexer::parse(markdown);
    println!("{:?}", elements);

    let tree = parser::parse(elements);
    println!("{:?}", tree);

    let html = generator::generate(tree);

    html
}
