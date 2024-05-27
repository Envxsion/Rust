//cd -> cargo new rsmain -> cargo build -> cargo run
use std::io;


fn main() {
    println!("Tech w/ Tim tutorial");
    let x = 8; //implicit assignment (no manual assignment of type, compiler stores it as int for us)
    println!("x is {}", x);

    let mut y = 5;
    y += 2;
    println!("y is {} but...", y); //can change as y is mutable
    let y = 14; //override var with same name
    println!("y is {} now", y);

    //nameshadowing
    {
        //Scope
        let x = x+2;
        println!("x is {} in this scope, used var from parent scope and added 2", x);

        //Type change
        let x = "hello";
        println!("x is {} in this scope and its been redefined", x);
    }

    let x = x*28;
    println!("x is {} outside of scope", x);

    //Constants
    const SEC_IN_MIN: u32 = 60; //creating a var not const, value & type cant change after def it
    //const SEC_IN_MIN: u32 = 100; throws an error as well, can't redefine it
    println!("constant {}", SEC_IN_MIN);

    //Data Types
    {
        //Primitive Data Types
        let _x: i32 = 2;
        /*
            i = signed int -> aka no. has a sign hence it can be negative and positive
            u - unsigned int -> aka positive vals only
            f - float which can be f32 or f64, f32 has more precision and its signed
            bool - T or F
            char 
            str 
            tuple() - fixed len seq of elements which is immutable
            array[]

        
         */
        let mut tup: (i8, bool, char) = (108, false, 's');
        //println!("tuple: {}", tup); doesnt work, u gotta print vals out one by one
        tup.2 = 'a';
        //tup = (108, false, 's', 'b'); cant add more elements as that changes type even tho its mutable
        println!("tuple: {}", tup.2);

        let arr: [i32; 5] = [1, 2, 3, 4, 5]; //def type for 5 elements in one go
        println!("arr: {}",arr[3])
    }

    //User input
    let mut inp = String::new(); //grabbing new function from module String 
    io::stdin().read_line(&mut inp).expect("failed to read line"); //mutable reference to the inp var
    println!("Hi, {}", inp); 


}
