use thiserror::Error;

#[derive(Error, Debug,Clone)]
pub enum Error{
    #[error("sha1_c: <wordlist.txt> <sha1_hash>\n
            subd_s: <domain.com> ")]
    CliUsage,
    
}