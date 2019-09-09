/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut len = 0;
    plain.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_numeric() { 
                c
            } else {
                (219 - c as u8) as char
            }
        } ).fold(String::new(), |mut i, j| {
            if len == 5 {
                i.push(' ');
                len = 0;
            }
            len += 1;
            i.push(j); 
            i
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_numeric() { 
                c
            } else {
                (219 - c as u8) as char
            }
        })
        .collect()
}
