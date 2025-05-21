#![no_main]
#![no_std]

use defmt_rtt as _; // global logger

pub use bsp::hal;
pub use feather_m0 as bsp;

pub mod init;
pub mod shared;

use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
