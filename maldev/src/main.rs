mod error;
mod sha1_crack;
mod subdomain; 

use std::env;
use std::error::Error;
use crate::error::Error as CustomError;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(CustomError::NoArgs.into());
    }

    match args[1].as_str() {
        "sha1_c" => {
            if args.len() != 4 {
                return Err(CustomError::CliUsage.into());
            }
            sha1_crack::sha1_c(&args[2], &args[3])?;
        },
        "subd_s" => {
            if args.len() != 3 {
                return Err(CustomError::CliUsage.into());
            }
            subdomain::subd_s(&args[2])?;
        },
        _ => {
            return Err(CustomError::CliUsage.into());
        },
    }

    Ok(())
}
