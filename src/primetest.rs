/*
 This program contains the prime number test to stress out all cores of the CPU.
*/

use std::time::Instant;
use std::ops::DivAssign;

pub fn prime_stress_test() {
    let num = 1;
    let mut primes = 0;

    let limit: i32 = 500000;

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

struct Score { points: f64 }

impl DivAssign<f64> for Score {
    fn div_assign(&mut self, rhs: f64) {
        self.points /= rhs;
    }
}

pub fn main() {
    println!("Prime number test!");
    let start = Instant::now();

    prime_stress_test();

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let mut score = Score { points: 1000.0 };
    let elapsed_int = 0;

}
