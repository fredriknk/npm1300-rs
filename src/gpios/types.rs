/// GPIOs available on the nPM1300
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Gpio {
    None = 0,
    Gpio0 = 1,
    Gpio1 = 2,
    Gpio2 = 3,
    Gpio3 = 4,
    Gpio4 = 5,
}

// Add conversion from u8
impl TryFrom<u8> for Gpio {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Gpio0),
            2 => Ok(Self::Gpio1),
            3 => Ok(Self::Gpio2),
            4 => Ok(Self::Gpio3),
            5 => Ok(Self::Gpio4),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<Gpio> for u8 {
    fn from(gpio: Gpio) -> Self {
        gpio as u8
    }
}

/// GPIOs polarity
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioPolarity {
    NotInverted = 0,
    Inverted = 1,
}

// Add conversion from u8
impl TryFrom<u8> for GpioPolarity {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotInverted),
            1 => Ok(Self::Inverted),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<GpioPolarity> for u8 {
    fn from(gpio_polarity: GpioPolarity) -> Self {
        gpio_polarity as u8
    }
}

/// GPIO mode configuration
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioMode {
    /// GPI Input
    GpiInput = 0,
    /// GPI Logic1
    GpiLogic1 = 1,
    /// GPI Logic0
    GpiLogic0 = 2,
    /// GPI Rising Edge Event
    GpiEventRise = 3,
    /// GPI Falling Edge Event
    GpiEventFall = 4,
    /// GPO Interrupt
    GpoIrq = 5,
    /// GPO Reset
    GpoReset = 6,
    /// GPO PwrLossWarn
    GpoPowerLossWarning = 7,
    /// GPO Logic1
    GpoLogic1 = 8,
    /// GPO Logic0
    GpoLogic0 = 9,
}

// Add conversion from u8 to GpioMode
impl TryFrom<u8> for GpioMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GpiInput),
            1 => Ok(Self::GpiLogic1),
            2 => Ok(Self::GpiLogic0),
            3 => Ok(Self::GpiEventRise),
            4 => Ok(Self::GpiEventFall),
            5 => Ok(Self::GpoIrq),
            6 => Ok(Self::GpoReset),
            7 => Ok(Self::GpoPowerLossWarning),
            8 => Ok(Self::GpoLogic1),
            9 => Ok(Self::GpoLogic0),
            _ => Err(()),
        }
    }
}

// Add conversion from GpioMode to u8
impl From<GpioMode> for u8 {
    fn from(gpio_mode: GpioMode) -> Self {
        gpio_mode as u8
    }
}

/// GPIO Drive Strength Configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioDriveStrength {
    /// 1mA
    Drive1mA = 0,
    /// 6mA
    Drive6mA = 1,
}

impl TryFrom<u8> for GpioDriveStrength {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Drive1mA),
            1 => Ok(Self::Drive6mA),
            _ => Err(()),
        }
    }
}

impl From<GpioDriveStrength> for u8 {
    fn from(gpio_drive_strength: GpioDriveStrength) -> Self {
        gpio_drive_strength as u8
    }
}

/// GPIO Pull-up Configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioPullUp {
    Disable = 0,
    Enable = 1,
}

impl TryFrom<u8> for GpioPullUp {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            _ => Err(()),
        }
    }
}

impl From<GpioPullUp> for u8 {
    fn from(gpio_pull_up: GpioPullUp) -> Self {
        gpio_pull_up as u8
    }
}

/// GPIO Pull-down Configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioPullDown {
    Disable = 0,
    Enable = 1,
}

impl TryFrom<u8> for GpioPullDown {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            _ => Err(()),
        }
    }
}

// Add conversion from u8
impl From<GpioPullDown> for u8 {
    fn from(gpio_pull_down: GpioPullDown) -> Self {
        gpio_pull_down as u8
    }
}

/// GPIO Open Drain Configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioOpenDrain {
    Disable = 0,
    Enable = 1,
}

impl TryFrom<u8> for GpioOpenDrain {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            _ => Err(()),
        }
    }
}

impl From<GpioOpenDrain> for u8 {
    fn from(gpio_open_drain: GpioOpenDrain) -> Self {
        gpio_open_drain as u8
    }
}

/// GPIO Debounce Configuration
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioDebounce {
    Disable = 0,
    Enable = 1,
}

impl TryFrom<u8> for GpioDebounce {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disable),
            1 => Ok(Self::Enable),
            _ => Err(()),
        }
    }
}

impl From<GpioDebounce> for u8 {
    fn from(gpio_debounce: GpioDebounce) -> Self {
        gpio_debounce as u8
    }
}

/// GPIO input status
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum GpioStatus {
    Low = 0,
    High = 1,
}

// Add conversion from u8
impl TryFrom<u8> for GpioStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Low),
            1 => Ok(Self::High),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<GpioStatus> for u8 {
    fn from(gpio_status: GpioStatus) -> Self {
        gpio_status as u8
    }
}
