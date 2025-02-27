#![no_std]
#![no_main]

//! Example demonstrating control of the NPM1300 PMIC's BUCK2 voltage regulator

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals,
    twim::{self, Twim},
};
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    buck::BuckVoltage,
    gpios::{Gpio, GpioPolarity},
    NPM1300,
};

bind_interrupts!(struct Irqs {
    SERIAL0 => twim::InterruptHandler<peripherals::SERIAL0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    
    let sdapin = p.P0_28;
    let sclpin = p.P0_29;

    let mut config = twim::Config::default();

    // Modify the configuration fields
    config.sda_pullup = true;
    config.scl_pullup = true;

    let twi = Twim::new(p.SERIAL0, Irqs, sdapin, sclpin, config);

    let mut npm1300 = NPM1300::new(twi, embassy_time::Delay);

}
