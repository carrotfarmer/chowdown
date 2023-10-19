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
