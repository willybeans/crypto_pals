extern crate base64;
use std::u8;
use base64::{Engine as _, engine::general_purpose};

/*
a byte contains 8 bits
therefore we convert each 4-bit segment to hex separately and concatenate them.
therefore we need 2 hexadecimals digits to create one byte

0010 = 2 (base 10) = 2 (base 16)
1101 = 13 (base 10) = d (base 16)

Therefore: 45 = 0010 1101 = 0x2d

https://www.baeldung.com/java-byte-arrays-hex-strings#:~:text=Now%2C%20let's%20convert%20a%20hexadecimal,digit%20into%20binary%20equivalent%20separately.

why is hex prefixed with 0x: 
https://stackoverflow.com/questions/2670639/why-are-hexadecimal-numbers-prefixed-with-0x#:~:text=It's%20a%20prefix%20to%20indicate,*16%5E0%20%3D%2025600.
It's a prefix to indicate the number is in 
hexadecimal rather than in some other base. 

*/

pub fn calculate(hex: String) -> String {
  // Make vector of bytes from octets
  // u8 0 -> 255, smalled integer type
  // what is an octet? 
  let mut bytes = Vec::new();
  // println!("hex: {:?}", hex);

  for i in 0..(hex.len()/2) {
    // hexadecimal string is : 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, A, B, C, D, E, and F
    // base64 is 0-9(10) + A-Z(24) + a-z(24) + (2) special chars +/
      //from_str_radix: Converts a string slice in a given base to an integer. here we are hexadecimal (16)
      // meaning our initial is hex, but we are converting it to base64
      // here a string slice, ie 1..5, grabs a fragment of a string, which is why the first result is 49
      let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
      // println!("--i:{:?}--of--{:?}-", i, (hex.len()/2));
      // println!("test op: {} {} {}", i, 2*i, 2*i+2);
      // println!("hex[]: {:?}", &hex[2*i .. 2*i+2]);
      // println!("str_radix: {:#?}", u8::from_str_radix("49", 16));
      // println!("res: {:?}", res); // is 73
      // println!("bytes array: {:?}", bytes);
      /*
      here our first string would be 49
      which then converts to SQ==
      which you now need to google how that happens exactly lol
       */

      match res {
          Ok(v) => bytes.push(v),
          Err(e) => println!("Problem with hex: {}", e),
      };
  };


  general_purpose::STANDARD.encode(&bytes) // now convert from Vec<u8> to b64-encoded String
  //https://base64.guru/learn/base64-characters -- explains the == as padding
}

/*
base 64 encoding algorithm: https://base64.guru/learn/base64-algorithm/encode

to encode a string to base 64
1) take all letters, here ABC, and make them into individuals
then convert them to their binary equivalent:
A -> 01000001
B -> 01000010
C -> 01000011

then you concatenate them all:  010000010100001001000011

then you split them again but by 6

010000
010100
001001
000011

then append 00 before each

00010000
00010100
00001001
00000011

then using and ascii table convert to its decimal value

16
20
9
3

Integer numbers obtained in the previous step are called 
“Base64 indices”. They are easy to remember, 
because it is a zero-based numbering, where each 
index corresponds to a Latin letter. It starts with the 
letter “A” in alphabetical order (i.e., A=0, B=1, C=2, D=3, and so on).
 For complete list, see
https://base64.guru/learn/base64-characters

the final string then is 
Q
U
J
D
*/