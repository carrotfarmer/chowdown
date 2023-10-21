fn main() {
    /* get input */
    let string = "# bruh";

    println!("{}", chowdown::parse(string.to_owned()));
}
