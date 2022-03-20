//
//  This is the main file for the rusty-riscy perf tool
//

use std::io;
use std::{thread, time};
use std::fs;

fn sleep() {
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}

fn print_logo() {
    print!("{}[2J", 27 as char); // to clear terminal screen

    let filename = "logo.txt";
    let logo_con = fs::read_to_string(filename)
        .expect("Failed to read the file");

    println!("{}",logo_con);
}

fn main() {
    print_logo();

    loop {
        println!("Enter the option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let trim_option = option.trim();
        let mut trim_flag = false;

        match trim_option.parse::<u32>() {
            Ok(i) => println!("You entered option number: {}", i),
            Err(..) => println!("This is not an integer!"),
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
