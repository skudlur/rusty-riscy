#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_rt::entry;
use riscv;

#[entry]
fn main() -> ! {
    // Read the contents of the x0 register
    let x0 = riscv::register::x0::read();

    // Write a value to the x1 register
    riscv::register::x1::write(0x12345678);

    // Read the contents of the mstatus register
    let mstatus = riscv::register::mstatus::read();

    // Print the contents of the registers
    println!("x0 = {:#x}", x0);
    println!("x1 = {:#x}", riscv::register::x1::read());
    println!("mstatus = {:#x}", mstatus.bits());

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
