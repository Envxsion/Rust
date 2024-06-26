use std::{
    error::Error, 
    fs::File,
    io::{BufRead, BufReader},
};
use sha1::Digest;
use crate::error::Error as CustomError; // Import the custom error module

const SHA1_HEX_LEN: usize = 40;

pub fn sha1_c(wordlist: &str, hash: &str) -> Result<(), Box<dyn Error>> {
    if hash.len() != SHA1_HEX_LEN {
        return Err(CustomError::InvalidHashLength { length: hash.len() }.into());
    }

    let wordlist_file = File::open(wordlist)?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_pass = line.trim();
        println!("{}", common_pass.to_string());
        if hash == &hex::encode(sha1::Sha1::digest(common_pass.as_bytes())) {
            println!("Found password: {}", common_pass);
            return Ok(());
        }
    }
    println!("Password not found");
    Ok(())
}
