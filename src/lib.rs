extern crate base64;
use std::str;
use std::ops::BitXor;

/// Frequencies of letters in English language.
const ENGLISH_LETTER_FREQ: [f32; 26] = [
    8.12,  // a
    1.49,  // b
    2.71,  // c
    4.32,  // d
    12.02, // e
    2.3,   // f
    2.03,  // g
    5.92,  // h
    7.31,  // i
    0.1,   // j
    0.69,  // k
    3.98,  // l
    2.61,  // m
    6.95,  // n
    7.68,  // o
    1.82,  // p
    0.11,  // q
    6.02,  // r
    6.28,  // s
    9.1,   // t
    2.88,  // u
    1.11,  // v
    2.09,  // w
    0.17,  // x
    2.11,  // y
    0.07,  // z
];

/// A wrapper for `Vec<u8>` so we can define our own trait implementations.
#[derive(Debug)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Return a string containing the bytes represented as base64.
    pub fn to_base64(&self) -> String {
        base64::encode(&self.0)
    }

    /// Return a string representation of the Bytes.
    pub fn to_str(&self) -> &str {
        str::from_utf8(&self.0).unwrap()
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
    /// Construct a new Bytes object from a string slice.
    pub fn from_str(s: &str) -> Bytes {
        Bytes(String::from(s).into_bytes())
    }
}

impl BitXor<Bytes> for Bytes {
    type Output = Bytes;

    /// xor each of the bytes. Will panic if the Bytes objects are not the same length.
    fn bitxor(self, rhs: Bytes) -> Bytes {
        assert!(self.0.len() == rhs.0.len());
        let mut ret = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            ret.push(self.0[i] ^ rhs.0[i]);
        };
        Bytes(ret)
    }
}

impl BitXor<u8> for Bytes {
    type Output = Bytes;

    /// xor each of the bytes in Byte with a single byte.
    fn bitxor(self, rhs: u8) -> Bytes {
        let mut ret = Vec::with_capacity(self.0.len());
        for i in 0..self.0.len() {
            ret.push(self.0[i] ^ rhs);
        };
        Bytes(ret)
    }
}

impl BitXor<Vec<u8>> for Bytes {
    type Output = Bytes;

    /// xor the bytes with a repeating key
    fn bitxor(self, rhs: Vec<u8>) -> Bytes {
        let mut ret = Vec::with_capacity(self.0.len());
        for (i,b) in self.0.iter().enumerate() {
            ret.push(self.0[i] ^ rhs[i % rhs.len()]);
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

impl Clone for Bytes {
    fn clone(&self) -> Bytes { Bytes (self.0.clone()) }
}

/// Return the frequency of letter bytes appearing in the Bytes object.
pub fn letters_freq(_bytes: &Bytes) -> [f32;26] {
    let mut ret: [f32;26] = [0.0; 26];
    let mut num_letters: f32 = 0.0;
    for b in _bytes.0.to_owned() {
        let b = b as usize;
        if b >= 65 && b < 91 {
            ret[b - 65] += 1.0;
            num_letters += 1.0;
        } else if b >= 97 && b < 123 {
            ret[b - 97] += 1.0;
            num_letters += 1.0;
        }
    }
    for r in ret.iter_mut() {
        if num_letters != 0.0 {
            *r *= 100.0 / num_letters;
        } else {
            *r = 0.0;
        }
    }
    ret
}

/// A distance function from the frequency of letters in the english language.
pub fn english_distance(freq:[f32;26]) -> f32 {
    let mut ret = 0.0;
    for (i, b) in freq.iter().enumerate() {
        ret += (b - ENGLISH_LETTER_FREQ[i]).abs();
    }
    ret
}

#[cfg(test)]
mod tests {
    use Bytes;
    use super::*;

    #[test]
    fn test_letters_freq() {
        assert_eq!(letters_freq(&Bytes::from_hex("4927")),
                   [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,
                    100.0, // letter 'i'
                    0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0]);
    }
    #[test]
    fn test_string_to_bytes() {
        assert_eq!(Bytes::from_str("hello"),
                   Bytes(vec![104, 101, 108, 108, 111]));
    }
}
