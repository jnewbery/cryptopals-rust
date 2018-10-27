extern crate base64;
use std::ops::BitXor;

/// A wrapper for `Vec<u8>` so we can define our own trait implementations.
#[derive(Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Return a string containing the bytes represented as base64.
    pub fn to_base64(&self) -> String {
        base64::encode(&self.0)
    }

    /// Construct a new Bytes object from a hex string slice.
    pub fn from_hex(hex: &str) -> Bytes {
        let mut ret = Vec::new();
        for i in 0..(hex.len()/2) {
            let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
            match res {
                Ok(v) => ret.push(v),
                Err(e) => println!("Problem with hex: {}", e),
            };
        };
        Bytes(ret)
    }
}

impl BitXor for Bytes {
    type Output = Bytes;

    /// xor each of the bytes. Both Bytes objects must be the same length.
    fn bitxor(self, rhs: Bytes) -> Bytes {
        let mut ret = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            ret.push(self.0[i] ^ rhs.0[i]);
        };
        Bytes(ret)
    }
}

impl PartialEq for Bytes {
    /// Bytes objects are equal if all the bytes are equal.
    fn eq(&self, other: &Bytes) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use Bytes;

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(Bytes::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").to_base64(),
                   String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
    }

    #[test]
    fn test_bytesxor() {
        assert_eq!((Bytes::from_hex("1c0111001f010100061a024b53535009181c") ^ Bytes::from_hex("686974207468652062756c6c277320657965")),
                   Bytes::from_hex("746865206b696420646f6e277420706c6179"));
    }
}
