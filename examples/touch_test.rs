#![no_std]
#![no_main]

use driver_cy8cmbR3108::{
    registers, CY8CMBR3108, CY8CMBR3XXX_CTRL_CMD_CALC_CRC, CY8CMBR3XXX_CTRL_CMD_RESET,
};
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    time::Rate,
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

    let delay = Delay::new();

    let mut touch = CY8CMBR3108::new(i2c, delay);

    for i in 0..5 {
        let is_device_ready = touch.ready();
        match is_device_ready {
            Ok(true) => {
                println!("CY8CMBR3108 Initialization successful!");
                break;
            }
            Ok(false) => {
                println!("CY8CMBR3108 Initialization failed: Device not ready after 3 attempts.")
            }
            Err(e) => println!("CY8CMBR3108 Initialization failed with error: {:?}", e),
        }
    }
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
            println!("CY8CMBR3108 Config successful!");
        }
        Ok(false) => {
            println!("CY8CMBR3108 Config failed,")
        }
        Err(e) => println!("CY8CMBR3108 Config failed with error: {:?}", e),
    }

    loop {
        delay.delay_millis(100);
        println!(
            "CY8CMBR3108 Button Status    : {:08b}",
            touch.get_button_status().unwrap()
        );
    }
}
