/// Buck regulator voltages available on the nPM1300
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum BuckVoltage {
    V1_0 = 0,
    V1_1 = 1,
    V1_2 = 2,
    V1_3 = 3,
    V1_4 = 4,
    V1_5 = 5,
    V1_6 = 6,
    V1_7 = 7,
    V1_8 = 8,
    V1_9 = 9,
    V2_0 = 10,
    V2_1 = 11,
    V2_2 = 12,
    V2_3 = 13,
    V2_4 = 14,
    V2_5 = 15,
    V2_6 = 16,
    V2_7 = 17,
    V2_8 = 18,
    V2_9 = 19,
    V3_0 = 20,
    V3_1 = 21,
    V3_2 = 22,
    V3_3 = 23,
    V3_30 = 24,
}

// Add conversion from u8 to BuckVoltage
impl TryFrom<u8> for BuckVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::V1_0),
            1 => Ok(Self::V1_1),
            2 => Ok(Self::V1_2),
            3 => Ok(Self::V1_3),
            4 => Ok(Self::V1_4),
            5 => Ok(Self::V1_5),
            6 => Ok(Self::V1_6),
            7 => Ok(Self::V1_7),
            8 => Ok(Self::V1_8),
            9 => Ok(Self::V1_9),
            10 => Ok(Self::V2_0),
            11 => Ok(Self::V2_1),
            12 => Ok(Self::V2_2),
            13 => Ok(Self::V2_3),
            14 => Ok(Self::V2_4),
            15 => Ok(Self::V2_5),
            16 => Ok(Self::V2_6),
            17 => Ok(Self::V2_7),
            18 => Ok(Self::V2_8),
            19 => Ok(Self::V2_9),
            20 => Ok(Self::V3_0),
            21 => Ok(Self::V3_1),
            22 => Ok(Self::V3_2),
            23 => Ok(Self::V3_3),
            24 => Ok(Self::V3_30),
            _ => Err(()),
        }
    }
}

// Add conversion from BuckVoltage to u8
impl From<BuckVoltage> for u8 {
    fn from(voltage: BuckVoltage) -> Self {
        voltage as u8
    }
}

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

// Add conversion from u8 to BuckVoltage
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

// Add conversion from BuckVoltage to u8
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

// Add conversion from u8 to BuckVoltage
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

// Add conversion from BuckVoltage to u8
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
