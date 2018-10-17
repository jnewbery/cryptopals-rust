extern crate base64;

pub fn hex_to_base64(hex: &str) -> String {
    base64::encode(&hex_to_bytes(hex))
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..(hex.len()/2) {
        let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };
    bytes
}

pub fn xor(a: &str, b: &str) -> Vec<u8> {
    let mut a = hex_to_bytes(a);
    let b = hex_to_bytes(b);
    for i in 0..a.len() {
        a[i] ^= b[i];
    };
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hex_to_base64() {
        assert_eq!(::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
                   String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
    }

    #[test]
    fn test_xor() {
        assert_eq!(::xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"),
                   ::hex_to_bytes("746865206b696420646f6e277420706c6179"));
    }
}
