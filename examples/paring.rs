#![allow(warnings)]
#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt as _;

use panic_halt as _;
use core::{convert::TryInto, ops::Range};
use cortex_m::asm::{self, delay};
use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::config::Config,
};

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;
use crc::{Crc, Algorithm, CRC_16_IBM_SDLC, CRC_32_ISCSI};

pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
const CUSTOM_ALG: Algorithm<u16> = Algorithm {
    width: 16,
    poly: 0x1021,
    init: 0xffff,//0xf078
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0xaee7,
    residue: 0x0000
};
#[entry]
fn main() -> ! {
let header = [0x00];
let data = [0x01,0x02,0x03,0x04];
    // use custom algorithm
let crc = Crc::<u16>::new(&CUSTOM_ALG);
let mut digest = crc.digest();
//digest.update(b"123456789");
digest.update(&header);

digest.update(&data);
let result = digest.finalize();
defmt::println!("{:02x}", result);
//no mac = d658  58d6 a729
//cmac 0x80  = b875  75b8  8a47
//rmac 0x40  = 614e        b19e
let mut echo_frame = [0x00, 0x04, 0x05, 0x06,0x84,0x31];

let mut host_key_slot_query = [0x14,0x17,0x00,0x00];
let mut hostkey_response = [0;4];
let mut put_attribute = [0x17,0x00,0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88,0x99,0xAA,0xBB,0xCC,0xDD,0xEE,0xFF,0x11,0x11,0x11,0x22,0x22,0x33,0x33,0x44,0x44,0x55,0x55,0x66,0x66,0x77,0x77,0x88,0x88];
let mut put_attribute_response = [0;4];
//let mut envelopekeyslot = [0x14,0x17,0x66,0x77]; 
//let mut envelopekeyslot = [0x14,0x17,0x77,0x66];// working
let mut envelopekeyslot = [0x14,0x17,0x99,0x88];
//let mut envelopekeyslot = [0x14,0x17,0x88,0x99];
let mut enveloperesponse = [0;7];
let mut envelopekey = [0x11,0x07,0x00,0x00,0xaf,0xb1];
let mut envelopekey_response = [0x00;3];
let mut rx_data: [u8;6] = [0;6];

//assert_eq!(val, 0xaee7);
    defmt::println!("Start");
    let dp = pac::Peripherals::take().unwrap();
    let mut addr1 =0;
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

    //defmt::println!("Start i2c scanning...");
    loop{
        i2c.write(0x20, &echo_frame);
   //let res =  i2c.write_read(0x20, &envelopekeyslot, &mut enveloperesponse).is_ok();
   delay(41000);//5msec   1msec = 5325 cycles
   let res = i2c.read(0x20, &mut rx_data).is_ok();
    defmt::println!("envelope !{}z",res);
    defmt::println!("envelopekey slot = {:?}",&rx_data); 
    delay(410000);//5msec
    }
    //loop{
    // i2c.write_read(0x0020, &echo_frame, &mut rx_data).is_ok();
    // defmt::println!("{:?}",rx_data); 
    //  i2c.write(addr1, &echo_frame);
    // delay(41000);//5msec
    // i2c.read(addr1, &mut rx_data).is_ok();
    //  defmt::println!("{:?}",rx_data);   
    //}

    loop {
        //asm::wfi();
    }
}
