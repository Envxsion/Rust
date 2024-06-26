use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("No arguments provided!!")]
    NoArgs,

    #[error("Usage\nsha1_c: <wordlist.txt> <sha1_hash>\nsubd_s: <domain.com>")]
    CliUsage,

    #[error("Length of SHA1 hash -> {length} <- is invalid")]
    InvalidHashLength { length: usize },
}
