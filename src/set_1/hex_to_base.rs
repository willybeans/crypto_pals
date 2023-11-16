extern crate base64;
use std::u8;
use base64::{Engine as _, engine::general_purpose};

pub fn calculate(hex: String) -> String {

  // Make vector of bytes from octets
  // u8 0 -> 255, smalled integer type
  // what is an octet? 
  let mut bytes = Vec::new();
  // println!("hex: {:?}", hex);

  for i in 0..(hex.len()/2) {
      //from_str_radix: Converts a string slice in a given base to an integer. here we are hexadecimal (16)
      // meaning our initial is hex, but we are converting it to base64
      // here a string slice, ie 1..5, grabs a fragment of a string, which is why the first result is 49
      let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);
      // println!("--i:{:?}--of--{:?}-", i, (hex.len()/2));
      // println!("test op: {} {} {}", i, 2*i, 2*i+2);
      // println!("hex[]: {:?}", &hex[2*i .. 2*i+2]);
      // println!("str_radix: {:#?}", u8::from_str_radix("72", 16));
      // println!("res: {:?}", res);

      match res {
          Ok(v) => bytes.push(v),
          Err(e) => println!("Problem with hex: {}", e),
      };
  };

  general_purpose::STANDARD.encode(&bytes) // now convert from Vec<u8> to b64-encoded String
}
