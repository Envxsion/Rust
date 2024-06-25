use std::{
    env, 
    fs::File,
    io::{BufRead, BufReader}
};
mod error;
pub use std::error::Error;
const SHA1_HEX_LEN: usize = 40;

pub fn sha1_c() -> Result<(), Box<dyn Error>>{

    let args: Vec<String> = env::args().collect(); //Vector is a resizable arr type

    if args.len() != 3 {
        return Err(<dyn Error>::CliUsage.into());
    }

    let xchash = args[2].trim();
    if xchash.len() != SHA1_HEX_LEN {
        let res = format!("Length of SHA1 hash {:?} is invalid",xchash.chars().count().to_string().as_str());
        return Err(res.into());
        
    }
    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines(){
        let line = line?.trim().to_string();
        println!("{}",line);
    }
    Ok(())

}