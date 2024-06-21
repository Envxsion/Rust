use std::{
    env, 
    error::Error, 
    fs::File,
    io::{BufRead, BufReader},
};
use sha1::Digest;

const SHA1_HEX_LEN: usize = 40;

pub fn sha1_c() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect(); //Vector is a resizable arr type

    if args.len() != 3 {
        println!("Usage: ");
        println!("sha1_c: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let xchash = args[2].trim();
    if xchash.len() != SHA1_HEX_LEN {
        let res = format!("Length of SHA1 hash {:?} is invalid",xchash.chars().count().to_string().as_str());
        return Err(res.into());
        
    }
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines(){
        let line = line?;
        let common_pass = line.trim();
        println!("{}",common_pass.to_string());
        if xchash == &hex::encode(sha1::Sha1::digest(common_pass.as_bytes())){
            println!("Found password: {}", common_pass);
            return Ok(());
        }
        
    }
    println!("Password not found");
    Ok(())

}