mod generator;
mod parser;

use generator::Generator;
use parser::Parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn parse(markdown: String) -> String {
    console_log!("hello");
    let elements = Parser::new(markdown).parse();
    println!("{:?}", elements);

    let html = Generator::new(elements).generate();

    html
}
