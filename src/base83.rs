use crate::Error;

static CHARACTERS: [u8; 83] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
    b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V',
    b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
    b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'#', b'$',
    b'%', b'*', b'+', b',', b'-', b'.', b':', b';', b'=', b'?', b'@', b'[', b']', b'^', b'_', b'{',
    b'|', b'}', b'~',
];

pub fn encode_into(value: u32, length: u32, s: &mut String) {
    for i in 1..=length {
        let digit: u32 = (value / u32::pow(83, length - i)) % 83;
        s.push(CHARACTERS[digit as usize] as char);
    }
}

pub fn decode(str: &str) -> Result<u64, Error> {
    // log_83(2^64) = 10.03
    if str.len() > 10 {
        panic!("base83::decode can only process strings up to 10 characters");
    }
    let mut value = 0;

    for byte in str.as_bytes() {
        let digit: usize = CHARACTERS
            .iter()
            .position(|r| r == byte)
            .ok_or(Error::InvalidBase83(*byte))?;
        value = value * 83 + digit as u64;
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{decode, encode_into};

    fn encode(value: u32, length: u32) -> String {
        let mut s = String::new();
        encode_into(value, length, &mut s);
        s
    }

    #[test]
    fn encode83() {
        let str = encode(6869, 2);
        assert_eq!(str, "~$");
    }

    #[test]
    fn decode83() {
        let v = decode("~$").unwrap();
        assert_eq!(v, 6869);
    }

    #[test]
    #[should_panic]
    fn decode83_too_long() {
        let _ = decode("~$aaaaaaaaa");
    }
}
