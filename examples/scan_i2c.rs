#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::{
    i2c::master::{Config, I2c},
    xtensa_lx_rt::entry,
};
use esp_println::println;
esp_bootloader_esp_idf::esp_app_desc!();

#[entry]
fn main() -> ! {
    // init CPU
    let mut config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let mut peripherals = esp_hal::init(config);

    let mut config = Config::default();
    let mut i2c = I2c::new(peripherals.I2C0.reborrow(), config)
        .unwrap()
        .with_sda(peripherals.GPIO47)
        .with_scl(peripherals.GPIO48);

    let mut buf: [u8; 1] = [0];
    for j in 0..2 {
        Delay::new().delay_millis(50);
        for i in 0..127 {
            let result = i2c.read(i, &mut buf);
            match result {
                Ok(_) => println!("address = {:02X}", i),
                Err(_) => {}
            }
        }
    }
    loop {}
}
