#![no_std]
#![no_main]

//! Example demonstrating the NPM1300 PMIC's vbat measurement and temp features

use embassy_executor::Spawner;
use embassy_nrf::{
    peripherals,
    bind_interrupts,
    twim::{self, Twim},
};

use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    NtcThermistorType,
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

    defmt::info!("Configuring TWIM...");
    let twi = Twim::new(p.SERIAL0, Irqs, sdapin, sclpin, config);
    
    let mut npm1300 = NPM1300::new(twi, embassy_time::Delay);
    defmt::info!("Configuring NTC Resistor...");
    let _ = npm1300.configure_ntc_resistance(NtcThermistorType::Ntc10K, Some(3380.0)).await;

    let vbat_measurement = npm1300.measure_vbat().await.unwrap();
    defmt::info!("VBAT: {=f32} mV", vbat_measurement);
    
    let _ = npm1300.measure_ntc().await;
    let ntc_temp = npm1300.get_ntc_measurement_result().await.unwrap();
    defmt::info!("NTC Temp: {=f32} C", ntc_temp);
}
