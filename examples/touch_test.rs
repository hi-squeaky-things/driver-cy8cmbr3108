#![no_std]
#![no_main]

use driver_cy8cmbr3108::CY8CMBR3108;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    xtensa_lx_rt::entry,
};
use esp_println::println;
esp_bootloader_esp_idf::esp_app_desc!();

#[entry]
fn main() -> ! {
    // init CPU
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let mut peripherals = esp_hal::init(config);

    let config = Config::default();
    let i2c = I2c::new(peripherals.I2C0.reborrow(), config)
        .unwrap()
        .with_sda(peripherals.GPIO47)
        .with_scl(peripherals.GPIO48);

    let delay = Delay::new();

    let mut touch = CY8CMBR3108::new(i2c, delay);
    let _ = touch.wake_up();

    //device id = 2563
    println!(
        "CY8CMBR3108 Device Identifier : {:?} (=2563)",
        touch.get_device_id().unwrap()
    );
    // revision = 0001
    println!(
        "CY8CMBR3108 Device Revision   : {:04} (=0001)",
        touch.get_device_revision().unwrap()
    );

    let is_config_ready = touch.write_configuration();
    match is_config_ready {
        Ok(true) => {
            println!("CY8CMBR3108 Configuration successful!");
        }
        Ok(false) => {
            println!("CY8CMBR3108 Configuration failed.")
        }
        Err(e) => println!("CY8CMBR3108 Configuration failed with error: {:?}", e),
    }

    loop {
        delay.delay_millis(100);
        println!(
            "CY8CMBR3108 Button Status    : {:08b}",
            touch.get_button_status().unwrap()
        );

        println!(
            "CY8CMBR3108 Proximity Status : {:08b}",
            touch.get_prox_status().unwrap() & 0b0000_0011
        );
    }
}
