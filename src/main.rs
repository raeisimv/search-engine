/*
 * This is an implementation of this paper:
 * https://jdrouet.github.io/posts/202503161800-search-engine-intro/
*/
use crate::cipher::Cipher;

mod cipher;
mod errors;

fn main() {
    let input = b"Hello world!";
    let k = b"My_32-lenth_Secret_Pa$$w0rd_1234";
    let c = Cipher::from_key(k).expect("invalid key");
    let enc = c.encrypt(input).expect("encryption error");
    let dec = c.decrypt(&enc).expect("decryption error");
    println!("decrypted: {}", String::from_utf8(dec).unwrap());
}
