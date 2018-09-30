#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
#![feature(alloc)]

#![no_std]

#[macro_use]
extern crate parity_codec_derive;
extern crate parity_codec as codec;
extern crate wee_alloc;

#[macro_use]
extern crate alloc;

use core::intrinsics;
use codec::{Encode, Decode};

mod ext;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
	unsafe {
		intrinsics::abort()
	}
}

#[alloc_error_handler]
pub extern fn oom(_: ::core::alloc::Layout) -> ! {
	unsafe {
		intrinsics::abort();
	}
}

#[derive(Encode, Decode)]
enum Action {
    Inc(u32),
    Get,
}

static COUNTER_KEY: ext::Key = ext::Key([1; 32]);

#[no_mangle]
pub extern "C" fn call() {
    let input = ext::input();
    let action = Action::decode(&mut &input[..]).unwrap();

    match action {
        Action::Inc(by) => {
            let mut counter = ext::get_storage(&COUNTER_KEY).and_then(|v| u32::decode(&mut &v[..])).unwrap_or(0);
            counter += by;
            ext::set_storage(&COUNTER_KEY, Some(&u32::encode(&counter)));
        }
        Action::Get => {
            let raw_counter = ext::get_storage(&COUNTER_KEY).unwrap_or(vec![]);
            ext::return_(&raw_counter);
        }
    }
}

// We need to define `deploy` function, so the wasm-build will produce a constructor binary.
#[no_mangle]
pub extern "C" fn deploy() {
}
