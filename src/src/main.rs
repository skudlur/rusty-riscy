//
//  This is the main file for the rusty-riscy perf tool
//

use std::io;
use std::{thread, time};

fn sleep() {
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}

fn main() {
    println!("rusty-riscy");

    loop {
        println!("Enter the option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let trim_option = option.trim();

        match trim_option.parse::<u32>() {
            Ok(i) => println!("You entered option number: {}", i),
            Err(..) => println!("This is not an integer: {}", trim_option),
        };

        if true {
            println!("paranoia testing in");
            sleep();
            println!("3");
            sleep();
            println!("2");
            sleep();
            println!("1");
            sleep();
            println!("paranoia in progress");
        }

        if option != "" {
            break;
        }
    }
}
