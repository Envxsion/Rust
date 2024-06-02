
//cd -> cargo new rsmain -> cargo build -> cargo run
/* 
If ni is 1, then the sequence terminates at ni.
If ni is even, then ni+1 = ni / 2.
If ni is odd, then ni+1 = 3 * ni + 1.
*/

pub fn collatz(mut n: i32) -> u32 {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 0 {
            println!("Iter {}: {} / 2 ", count, n);
            n = n / 2;
            println!("Result: {}", n);
        } else {
            println!("Iter {}: 3 * {} + 1 ", count, n);
            n = 3 * n + 1;
            println!("Result: {}", n);
        }
        count += 1;
    }
    count
}
