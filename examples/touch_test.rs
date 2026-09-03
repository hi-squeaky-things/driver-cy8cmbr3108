#![no_std]
#![no_main]

use driver_cy8cmbr3108::CY8CMBR3108;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
//
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, pubsub::PubSubChannel,
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use embedded_hal_async::delay;
use esp_alloc::heap_allocator;
use esp_backtrace as _;

use esp_hal::delay::Delay;
use esp_hal::i2c::master::I2c;
use esp_hal::peripherals::Peripherals;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::Async;

use esp_println::println;
use static_cell::StaticCell;

//

esp_bootloader_esp_idf::esp_app_desc!();
type I2c1Bus = Mutex<NoopRawMutex, esp_hal::i2c::master::I2c<'static, Async>>;

//type I2cDeviceType = i2c_bus::RefCellDevice<'static, esp_hal::i2c::master::I2c<'static, Async>>;

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // init CPU
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let mut peripherals = esp_hal::init(config);
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(timg0.timer0);

    let config = esp_hal::i2c::master::Config::default();
    let i2c = I2c::new(peripherals.I2C0, config)
        .unwrap()
        .with_sda(peripherals.GPIO47)
        .with_scl(peripherals.GPIO48)
        .into_async();

    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(i2c));

    //  let delay = esp_hal::delay::Delay::new();

    let _ = spawner.spawn(test(i2c_bus));
    loop {
        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::task]
pub async fn test(i2c_bus: &'static I2c1Bus) {
    // function body

    let i2c_dev = I2cDevice::new(i2c_bus);

    let delay = Delay::new();

    let mut touch = CY8CMBR3108::new(i2c_dev, delay);
    let _ = touch.wake_up().await;

    //device id = 2563
    println!(
        "CY8CMBR3108 Device Identifier : {:?} (=2563)",
        touch.get_device_id().await.unwrap()
    );
    // revision = 0001
    println!(
        "CY8CMBR3108 Device Revision   : {:04} (=0001)",
        touch.get_device_revision().await.unwrap()
    );

    let is_config_ready = touch.write_configuration().await;
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
        Timer::after(Duration::from_millis(100)).await;
        println!(
            "CY8CMBR3108 Button Status    : {:08b}",
            touch.get_button_status().await.unwrap()
        );

        println!(
            "CY8CMBR3108 Proximity Status : {:08b}",
            touch.get_prox_status().await.unwrap() & 0b0000_0011
        );
    }
}
