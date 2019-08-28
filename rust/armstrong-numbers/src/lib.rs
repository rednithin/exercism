pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    num == num_string
        .chars()
        .map(|x| (x as u32 - 48).pow(num_string.len() as u32))
        .fold(0, |sum, x| sum + x)
}
