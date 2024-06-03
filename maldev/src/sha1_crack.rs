use std::{
    env,
    error::Error,
};

const SHA1_HEX_LEN: usize = 40;

pub fn sha1_c() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect(); //Vector is a resizable arr type

    if args.len() != 3 {
        println!("Usage: ");
        println!("sha1_c: <wordlist.txt> <sha1_hash>");
        return;
    }

    let xchash = args[2].trim();
    if xchash.len() != SHA1_HEX_LEN {
        println!("Invalid SHA1 hash length: {}", xchash.len());
        return;
    }
    Ok(())

}