# rp2040_min

The smallest thing that compiles to and RP2040, turns on an LED on the specified pin.

## steps
- add 'memory.x' mem linker file

- add the following to Cargo.toml:
>[dependencies]\
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"] }\
cortex-m-rt = "0.7.5"\
panic-halt = "1.0.0"\
rp2040-pac = "0.6.0"

- create .cargo/config.toml, add
>[build]\
target = "thumbv6m-none-eabi"\
\
[target.thumbv6m-none-eabi]\
rustflags = [\
    "-C", "link-arg=--nmagic",\
    "-C", "link-arg=-Tlink.x",\
    "-C", "link-arg=-Tdefmt.x",\
    "-C", "no-vectorize-loops",\
]\
\
[target.'cfg(all(target_arch = "arm", target_os = "none"))']\
runner = "elf2uf2-rs -d"

- minimal main.rs:
>#![no_std]\
>#![no_main]\
\
use panic_halt as _;\
\
>#[cortex_m_rt::entry]
fn main() -> ! {\
    loop{}\
}

- for error "rust-lld: error: cannot find linker script defmt.x", type "cargo add defmt" in cmd to fix