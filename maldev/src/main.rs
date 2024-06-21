mod sha1_crack;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    //cargo run wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
    sha1_crack::sha1_c().map_err(|err| println!("{:?}", err)).ok();

    // Smart Pointers
    let b = Box::new(5);
    println!("b = {}", b);

    // Define a recursive enum for a linked list
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),  // Cons is short for "construct"
        Nil,  // Represents the end of the list
    }

    // Create a linked list: 1 -> 2 -> 3 -> Nil
    let list = List::Cons(1,
        Box::new(List::Cons(2,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));

    println!("list = {:?}", list);

    //Lifetime Annotations
    let string1 = String::from("COuldbeBiggeR");
    let string2 = String::from("AmIBig");
    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest string is {}", result);

}
