//
// This program contains the prime number test to stress out all cores of the CPU.
//

use std::io;
use std::thread;
use std::time::Instant;

pub fn prime_stress_test() {
    let mut num = 1;
    let mut primes = 0;

    let mut limit: i32 = 500000;

    for num in 0..limit {
        let mut i = 2;
        while i <= num {
            if num % i == 0 {
                break;
            }
            i+=1;
        }
        if i == num {
            primes+=1;
        }
    }
}

pub fn main() {
    println!("Prime number test!!");
    let mut start = Instant::now();

    prime_stress_test();

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let mut score = 0;
    let mut elapsed_int = 0;

}
