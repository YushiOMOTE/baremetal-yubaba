#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;
use crate::{allocator::init_heap, console::new_console_640x480, time::rdtsc};
use bootloader::BootInfo;
use log::*;

mod allocator;
mod console;
mod keyboard;
mod time;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    com_logger::init();

    init_heap(boot_info).unwrap();

    let mut console = new_console_640x480();
    console.reset();
    console.print("契約書だよ。そこに名前を書きな。\n");

    let input = console.readline();
    if input.is_empty() {
        console.print(&format!("フン.  Segmentation fault"));
        loop {}
    }

    let idx = rdtsc() as usize % input.len();
    let nickname = input.chars().nth(idx).unwrap();

    console.print(&format!(
        "\nフン。{}というのかい。\n贅沢な名だねぇ。",
        input
    ));
    console.print(&format!(
        "今からお前の名前は{0}だ。\nいいかい、{0}だよ。分かったら返事をするんだ、{0}!!\n",
        nickname
    ));

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("panic: {:?}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    error!("allocation error: {:?}", layout);
    loop {}
}
