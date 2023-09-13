#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

extern crate wee_alloc; // Ainanenane, should be installed

use alloc::vec::Vec;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static moo: Vec<i32> = vec![1, 2, 3, 4, 5];

#[no_mangle]
pub extern "C" fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
