extern crate cryptopals;
use cryptopals::*;

#[test]
fn challenge1() {
    //! # Convert hex to base64
    //!
    //! The string:
    //!
    //! `49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d`
    //!
    //! Should produce:
    //!
    //! `SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t`
    //!
    //! So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
    assert_eq!(Bytes::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").to_base64(),
               String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"));
}

#[test]
fn challenge2() {
    //! # Fixed XOR
    //!
    //! Write a function that takes two equal-length buffers and produces their XOR combination.
    //!
    //! If your function works properly, then when you feed it the string:
    //!
    //! `1c0111001f010100061a024b53535009181c`
    //!
    //! ... after hex decoding, and when XOR'd against:
    //!
    //! `686974207468652062756c6c277320657965`
    //!
    //! ... should produce:
    //!
    //! `746865206b696420646f6e277420706c6179`
    assert_eq!((Bytes::from_hex("1c0111001f010100061a024b53535009181c") ^ Bytes::from_hex("686974207468652062756c6c277320657965")),
               Bytes::from_hex("746865206b696420646f6e277420706c6179"));
}

#[test]
fn challenge3() {
    //! # Single-byte XOR cipher
    //!
    //! The hex encoded string:
    //!
    //! `1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736`
    //!
    //! ... has been XOR'd against a single character. Find the key, decrypt the message.
    //!
    //! You can do this by hand. But don't: write code to do it for you.
    //!
    //! How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
    let cyphertext = Bytes::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let mut best_score = 100.0;
    let mut best_bytes = Bytes::from_hex("");
    for i in 0..255 {
        let mut new_bytes = cyphertext.clone();
        let mut new_bytes = new_bytes ^ i;
        let d = english_distance(letters_freq(&new_bytes));
        if d < best_score {
            best_score = d;
            best_bytes = new_bytes;
        }
    }
    assert_eq!(best_bytes.to_str(), "Cooking MC\'s like a pound of bacon")
}

#[test]
fn challenge5() {
    //! # Implement repeating-key XOR
    //!
    //!  Here is the opening stanza of an important work of the English language:
    //!
    //!  > Burning 'em, if you ain't quick and nimble
    //!  > I go crazy when I hear a cymbal
    //!
    //!  Encrypt it, under the key "ICE", using repeating-key XOR.
    //!
    //!  In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.
    //!
    //!  It should come out to:
    //!
    //!  0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
    //!  a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
    //!
    //!  Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.

    let plaintext = Bytes::from_str("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal");
    let key = String::from("ICE").into_bytes();
    assert_eq!(plaintext ^ key, Bytes::from_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"));
}
