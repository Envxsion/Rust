//cd -> cargo new rsmain -> cargo build -> cargo run
use std::io;
mod collatzseq;

fn main() {
    println!("Imported pub fn from another file with mod!");
    
    println!("Implicit assignment -> ");
    let x = 8; //implicit assignment (no manual assignment of type, compiler stores it as int for us)
    println!("x is {}", x);


    println!("Mutable Variable -> ");
    let mut y = 5;
    y += 2;
    println!("y is {} but...", y); //can change as y is mutable
    println!("Override Mutable Var -> ");
    let y = 14; //override var with same name
    println!("y is {} now", y);

    //nameshadowing
    println!("New Scope -> ");
    {
        //Scope
        println!("Var inheritence from parent scope -> ");
        let x = x+2;
        println!("x is {} in this scope, used var from parent scope and added 2", x);

        //Type change
        println!("Type change -> ");
        let x = "hello";
        println!("x is {} in this scope and its been redefined", x);
    }

    println!("Outside the scope -> ");
    let x = x*28;
    println!("x is {} outside of scope", x);

    //Constants
    println!("Constants -> ");
    const SEC_IN_MIN: u32 = 60; //creating a var not const, value & type cant change after def it
    //const SEC_IN_MIN: u32 = 100; throws an error as well, can't redefine it
    println!("constant {}", SEC_IN_MIN);

    //Data Types
    println!("Data Types -> ");
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
        println!("Tuples -> ");
        let mut tup: (i8, bool, char) = (108, false, 's');
        //println!("tuple: {}", tup); doesnt work, u gotta print vals out one by one
        tup.2 = 'a';
        //tup = (108, false, 's', 'b'); cant add more elements as that changes type even tho its mutable
        println!("tuple: {}", tup.2);
        
        println!("Arrays -> ");
        let arr:[i32;5] = [1, 2, 3, 4, 5]; //def type for 5 elements in one go
        println!("arr: {}", arr[3]);
        
        println!("Loop over arr -> ");
        for i in 0..arr.len() {
            println!("{}", i);
            for j in 2..arr.len() {
                println!("{}", arr[j]);
            }
        }
        
    }

    //User input
    println!("User Input -> ");
    let mut inp = String::new(); //grabbing new function from module String 
    io::stdin().read_line(&mut inp).expect("failed to read line"); //mutable reference to the inp var
    println!("Hi, {}", inp); 
    let z = x/y; 
    println!("{}/{}={}",x,y,z);

    //Type casting
    println!("Type Casting -> ");
    let a: i64 = 1280000;
    let b: i32 = 2648;
    let z = a/b as i64;
    println!("{}", z);
    
    //IMporting a file
    println!("Using imported function -> ");
    collatzseq::collatz(5);
    assert_eq!(collatzseq::collatz(11), 15);
    
    //Shared references
    println!("Shared References -> ");
    {
        let a = 'A';
        let b = 'B';
        let mut r: &char = &a;
        println!("r: {}", r);
        r = &b;
        println!("r: {}", r);

        println!("Pointer modifying a part of tuple -> ");
        let mut point = (1,2);
        let x_coord = &mut point.0;
        *x_coord = 20;
        println!("Point: {point:?}");

        //Slices
        println!("Slices -> ");
        let arr = [1, 2, 3, 4, 5,6,7,8,9,0];
        let slice = &arr[2..5];
        println!("Arr: {arr:?}, Slice: {slice:?}")
    }
    //Strings
    println!("Geometry -> ");
    {
        // Calculate the magnitude of a vector by summing the squares of its coordinates
        // and taking the square root. Use the `sqrt()` method to calculate the square
        // root, like `v.sqrt()`.

        fn magnitude(vector: &[f64; 3]) -> f64 {
            let mut squaredvec = 0.0;
            for coord in vector{
                squaredvec += coord*coord;
            }
            squaredvec.sqrt()
        }

        // Normalize a vector by calculating its magnitude and dividing all of its
        // coordinates by that magnitude.


        fn normalize(vector: &mut [f64;3]) {
            let magnitude = magnitude(vector);
            for item in vector{
                *item /= magnitude;
            }
        }

        // Use the following `main` to test your work.

        fn mainfunc() {
            println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
        
            let mut v = [1.0, 2.0, 9.0];
            println!("Magnitude of {v:?}: {}", magnitude(&v));
            normalize(&mut v);
            println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
        }
        mainfunc()
    }


}
