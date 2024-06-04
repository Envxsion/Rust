mod sha1_crack;

fn main() {
    sha1_crack::sha1_c().map_err(|err| println!("{:?}", err)).ok();
}
