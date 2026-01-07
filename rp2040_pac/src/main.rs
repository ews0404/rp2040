#![no_std]
#![no_main]

const LED_PIN: usize = 25;

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {

    // just grab everything
    let pac = unsafe { rp2040_pac::Peripherals::steal() };

    // set LED pin function to SIO (should be modify?)
    pac.IO_BANK0.gpio(LED_PIN).gpio_ctrl().write(|w| {
        w.funcsel().sio();
        w
    });

    // enable LED pin output
    pac.SIO.gpio_oe_set().write(|w| unsafe {
        w.bits(1 << LED_PIN);
        w
    });

    // set LED pin output
    pac.SIO.gpio_out_set().write(|w| unsafe {
        w.bits(1 << LED_PIN);
        w
    });

    loop {}
}
