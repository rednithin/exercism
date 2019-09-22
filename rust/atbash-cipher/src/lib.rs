/// "Encipher" with the Atbash cipher.

const ASCII_SMALL_A: u8 = 97;
const ASCII_SMALL_Z: u8 = 122;
const ASCII_SUM: u8 = ASCII_SMALL_A + ASCII_SMALL_Z;

pub fn encode(plain: &str) -> String {
    let mut len = 0;
    plain
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                if c.is_numeric() {
                    return Some(c);
                } else {
                    return Some((ASCII_SUM - c as u8) as char);
                }
            }
            None
        })
        .fold(
            String::with_capacity(plain.len() as usize + plain.len() / 5 as usize),
            |mut i, j| {
                if len == 5 {
                    i.push(' ');
                    len = 0;
                }
                len += 1;
                i.push(j);
                i
            },
        )
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                if c.is_numeric() {
                    return Some(c);
                } else {
                    return Some((ASCII_SUM - c as u8) as char);
                }
            }
            None
        })
        .collect()
}
