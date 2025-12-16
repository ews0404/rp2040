#![no_std]
#![no_main]

const LED_PIN: u32 = 25;

use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let pac = unsafe { rp2040_pac::Peripherals::steal() };

    // // Configure pin 25 for GPIO
    // pac.PADS_BANK0.gpio[25].write(|w| {
    //     // Output Disable off
    //     w.od().clear_bit();
    //     // Input Enable on
    //     w.ie().set_bit();
    //     w
    // });
    // pac.IO_BANK0.gpio[25].gpio_ctrl.write(|w| {
    //     // Map pin 25 to SIO
    //     w.funcsel().sio_0();
    //     w
    // });

    pac.SIO.gpio_oe_set().write(|w| unsafe {
        w.bits(1 << LED_PIN);
        w
    });

    pac.SIO.gpio_out_set().write(|w| unsafe {
        w.bits(1 << LED_PIN);
        w
    });

    loop {}
}
