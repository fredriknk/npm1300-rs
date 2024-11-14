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
