mod sha1_crack;

fn main() {
    //cargo run wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
    sha1_crack::sha1_c().map_err(|err| println!("{:?}", err)).ok();
}
