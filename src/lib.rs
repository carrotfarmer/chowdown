mod generator;
mod parser;

use generator::Generator;
use parser::Parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(markdown: String) -> String {
    let elements = Parser::new(markdown).parse();
    println!("{:?}", elements);

    let html = Generator::new(elements).generate();

    html
}
