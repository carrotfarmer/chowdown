fn main() {
    /* get input */
    let string = "`hello`\n";

    println!("{}", chowdown::parse(string.to_owned()));
}
