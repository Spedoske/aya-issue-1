#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_bpf::macros::map;
use aya_bpf::maps::LruHashMap;
use aya_log_ebpf::info;

#[map]
static TABLE: LruHashMap<u32, u32> = LruHashMap::with_max_entries(256, 0);

#[xdp]
pub fn foo(ctx: XdpContext) -> u32 {
    match try_foo(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_foo(ctx: XdpContext) -> Result<u32, u32> {
    // TABLE.insert(&0, &0, 0); // works
    TABLE.insert(&0, &0, 0).unwrap(); // will fail
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
