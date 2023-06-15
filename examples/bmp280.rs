#![allow(warnings)]
#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;
extern crate bmp280_ehal;
use core::{convert::TryInto, ops::Range};
use cortex_m::asm::{self, delay};
use cortex_m_rt::entry;
use panic_probe as _;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut addr1 = 0;
    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8.internal_pull_up(true);
    let sda = gpiob.pb9.internal_pull_up(true);
    defmt::println!("starting ....");
    // 3) Configure I2C perihperal channel
    // We're going to use I2C1 since its pins are the ones connected to the I2C interface we're using
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the i2c device peripheral handle and instantiate a transmitter instance using extension trait
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );

    defmt::println!("before bmp init..");
    // to create sensor with default configuration:
    let mut bmp = bmp280_ehal::BMP280::new(i2c).unwrap();
    defmt::println!("after bmp init");

    loop {
        let mut data: [u8; 24] = [0; 24];
        // to get pressure:
        let temp = bmp.temp();
        let pres = bmp.pressure();
        //let _ = i2c.write_read(0x76, &[0xFA as u8],&mut data);
        defmt::println!("TEMP....");
        defmt::println!("{:?}", temp);
        //delay(41000);
        let mut data: [u8; 24] = [0; 24];
        //let _ = i2c.write_read(0x76, &[0xF7 as u8],&mut data);

        defmt::println!("PRES....");
        defmt::println!("{:?}", pres);
    }
}
