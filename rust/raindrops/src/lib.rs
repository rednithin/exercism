pub fn raindrops(n: u32) -> String {
    let mut output = String::from("");
    if n % 3 == 0 {
        output.push_str("Pling");
    }
    if n % 5 == 0 {
        output.push_str("Plang");
    }
    if n % 7 == 0 {
        output.push_str("Plong");
    }
    if output.len() == 0 {
        output.push_str(&n.to_string()[..]);
    }
    output
}
