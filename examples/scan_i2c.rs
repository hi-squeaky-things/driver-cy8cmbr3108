// This code scans the I2C bus to detect connected devices.
// It initializes I2C on GPIO47 (SDA) and GPIO48 (SCL), then checks all 7-bit addresses (0-127).
// When a device responds, its address is printed. The scan is performed twice with a 50ms delay between scans.

#![no_std]  
#![no_main]  

//
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, pubsub::PubSubChannel, watch::Watch};
use embassy_time::{Duration, Timer};
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock, ram,
    system::{Cpu, CpuControl, Stack},
};
use esp_println::println;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::timer::timg::TimerGroup;

//

esp_bootloader_esp_idf::esp_app_desc!();  


#[esp_rtos::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    // Initialize CPU with 240MHz clock
    let config_cpu = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let mut peripherals = esp_hal::init(config_cpu);
 let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(timg0.timer0);


    // Configure I2C with default settings
    let config_i2c = Config::default();
    // Initialize I2C peripheral on I2C0 with SDA on GPIO47 and SCL on GPIO48
    let mut i2c = I2c::new(peripherals.I2C0.reborrow(), config_i2c)
        .unwrap()
        .with_sda(peripherals.GPIO47)
        .with_scl(peripherals.GPIO48);

    // Buffer for reading data (1 byte)
    let mut buf: [u8; 1] = [0];
    // Scan I2C bus twice with 50ms delay between scans
    for j in 0..2 {
         Timer::after(Duration::from_millis(50)).await;

        // Check all possible 7-bit I2C addresses (0-127)
        for i in 0..127 {
            // Attempt to read from current address
            let result = i2c.read(i, &mut buf);
            match result {
                Ok(_) => println!("Found an I2C device @ {:02X}", i),  // Print address if device responds
                Err(_) => {}  // Ignore errors (no device at this address)
            }
        }
    }
    // Infinite loop to keep the program running
    loop {}
}