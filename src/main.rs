use std::{fs, io};

fn gpio_test() {println!("gpio test");}
fn reg_test() {println!("reg test");}
fn alu_test() {println!("alu test");}
fn mem_test() {println!("mem test");}

fn logo_display() {
    let filename = "logo.txt";
    let logo_con = fs::read_to_string(filename)
        .expect("Failed to read the file");
    println!("{}",logo_con);
}


fn main() {
    logo_display();
    let mut choice = String::new(); 
    loop {
        println!("Operations available:");
        println!("1.GPIO test");
        println!("2.Register test");
        println!("3.ALU test");
        println!("4.Memory test");
        println!("Choose the operation:");
        
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the choice");

        let trim_choice = choice.trim().parse::<u32>().expect("Invalid choice!");
        
        match trim_choice {
            1 => gpio_test(),
            2 => reg_test(),
            3 => alu_test(),
            4 => mem_test(),
            _ => println!("Invalid choice"),
        }
    }
}


