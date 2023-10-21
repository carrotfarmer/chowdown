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

    println!("{}", chowdown::parse(string.to_owned()));
}
