fn main() {
    let string = "hello, not *quite* sure if i got it **completely** right yet..\nWhat if i... *didn't close it >:)\n";

    println!("{}", chowdown::parse(string.to_owned()));
}
