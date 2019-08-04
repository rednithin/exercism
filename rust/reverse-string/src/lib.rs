pub fn reverse(input: &str) -> String {
    let mut output = String::from("");
    for c in input.chars().rev() {
        output.push(c);
    }
    output
}
