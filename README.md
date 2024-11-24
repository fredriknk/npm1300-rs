# Rust nPM1300 PMIC Driver

A `no_std`, async Rust driver for the [Nordic nPM1300](https://www.nordicsemi.com/Products/nPM1300) Power Management IC (PMIC).  
This crate provides both low-level register access and a high-level API for managing PMIC functions.

## Features

- `no_std` support for embedded environments
- Async interface
- [`defmt`](https://github.com/knurling-rs/defmt) support for logging (optional)
- Uses the [`device-driver`](https://docs.rs/device-driver/) toolkit for low-level register generation

## API Coverage

> [!IMPORTANT]
> The driver is currently under development, and the API may change.

| **Component**                             | **Low-Level API** | **High-Level API** |
| ----------------------------------------- | :---------------: | :----------------: |
| SYSREG — System regulator                 |        ❌         |         ❌         |
| CHARGER — Battery charger                 |        ❌         |         ❌         |
| BUCK — Buck regulators                    |        ✅         |         ✅         |
| LOADSW/LDO — Load switches/LDO regulators |        ❌         |         ❌         |
| LEDDRV — LED drivers                      |        ❌         |         ❌         |
| GPIO — General-purpose I/O                |        ✅         |         ✅         |
| System Monitor                            |        ❌         |         ❌         |
| TIMER — Timer/monitor                     |        ❌         |         ❌         |
| Event and interrupt                       |        ❌         |         ❌         |
| Reset and error                           |        ❌         |         ❌         |
| Fuel gauge                                |        ❌         |         ❌         |

> [!WARNING]
> Most functions in this driver have been tested, though not exhaustively; contributions are welcome to help identify and fix any remaining bugs.

> [!NOTE]
> This crate does not provide a synchronous API and there are no plans to add one. Contributions are welcome!

## Usage

To use this crate, import it and initialize the driver with your I2C peripheral. Example code for the nRF52840 using the [`embassy`](https://github.com/embassy-rs/embassy) async framework is available in the [`examples`](examples) directory.

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    peripherals,
    twim::{self, Twim},
};
use {defmt_rtt as _, panic_probe as _};

use npm1300_rs::{
    types::BuckVoltage,
    NPM1300,
};

bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let config = twim::Config::default();

    let twi = Twim::new(p.TWISPI0, Irqs, p.P0_07, p.P0_12, config);

    let mut npm1300 = NPM1300::new(twi);
    let _ = npm1300.set_buck2_normal_voltage(BuckVoltage::V1_8).await;
    let _ = npm1300.enable_buck2().await;
}
```

## Support

For questions, issues, feature requests, and contributions, please open an issue on GitHub.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

By contributing, you agree that your work will be dual-licensed as above, without additional terms or conditions.

This project follows the [conventional commits](https://www.conventionalcommits.org) specification for commit messages.
