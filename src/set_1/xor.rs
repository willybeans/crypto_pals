// https://en.wikipedia.org/wiki/XOR_cipher
// https://stackoverflow.com/questions/14526584/what-does-the-xor-operator-do
// https://cryptopals.com/sets/1/challenges/2
use hex;

pub fn convert(first: String, second: String) ->  String {
  println!("convert1 {:?}", first);
  println!("convert2 {:?}", second);

// hex::decode
// hex encoding is converting a string into heaxadecimal
// decoding is into 'raw bytes'
// https://web.cecs.pdx.edu/~harry/compilers/ASCIIChart.pdf
// this chart you can see the conversion between 'unsigned' and 'hex'
// so for every 2 char's of hex, you get the associated 'unsigned' output
  let byte1 = hex::decode(first).unwrap();
  let byte2 = hex::decode(second).unwrap();
  println!("byte1 and 2: {:?} ::: {:?}", byte1 , byte2);
  println!("byte1 and 2: {:?} ::: {:?}", byte1 , byte2);
  println!("test: {:?}", hex::encode("1"));

  // trying to understand exactly how zip works
  let xor_bytes: Vec<u8> = byte1
  .iter()
  .zip(byte2.iter())
  .map(|(&b1, &b2)| {
    // println!("map1: {:?} {:?}", b1 , b2);
    // println!("map2: {:?}", b1 ^ b2);
    b1 ^ b2
  })
  .collect();
hex::encode(xor_bytes)
  // return String::from("");
}