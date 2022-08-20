// MIT License
//
// Copyright (c) 2022 Suhas KV
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
// 
// This is the main file for the rusty-riscy perf tool
//

use std::io;
use std::{thread, time};
use std::fs;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};

pub mod primetest;

fn sys_info() {
    /*
        System information initial list
    */

    let mut sys = System::new_all();

    let sys_name = sys.name();
    let sliced_sys_name = &sys_name[0..4];

    println!("System name:             {:?}", sliced_sys_name);
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    println!("NB CPUs: {}", sys.cpus().len());

    println!("=> system:");
    // RAM and swap information:
    let mut mb_total_memory = 0;
    let mut mb_used_memory = 0;
    mb_total_memory = (sys.total_memory() / 1000) - 1125;
    mb_used_memory = (sys.used_memory() / 1000) - 500; 
    println!("total memory: {} MB", mb_total_memory);
    println!("used memory : {} MB", mb_used_memory);

    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
}

fn sleep() {

    /* function to delay tests */

    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}

fn print_logo() {

    /* function to print logo */

    print!("{}[2J", 27 as char); // to clear terminal screen

    let filename = "logo.txt";
    let logo_con = fs::read_to_string(filename)
        .expect("Failed to read the file");

    println!("{}",logo_con);
}

fn main() {
    
    /* main function that calls all initial functions */

    print_logo();
    sys_info();

    loop {
        println!("Enter the option: ");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let trim_option = option.trim();
        let trim_flag = false;

        match trim_option.parse::<u32>() {
            Ok(i) => println!("You entered option number: {}", i),
            Err(..) => println!("This is not an integer!"),
        };

        match trim_option {
            "1" => primetest::main(),
            _ => println!("Enter a valid option!"),
        }
    }
}
