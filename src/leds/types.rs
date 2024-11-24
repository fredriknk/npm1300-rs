/// LED mode configuration
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum LedMode {
    /// Error condition from charger
    ChargingError = 0,
    /// Charging indicator (on during charging)
    Charging = 1,
    /// Driven by host
    Host = 2,
    /// Not used
    NotUsed = 3,
}

// Add conversion from u8
impl TryFrom<u8> for LedMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ChargingError),
            1 => Ok(Self::Charging),
            2 => Ok(Self::Host),
            3 => Ok(Self::NotUsed),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<LedMode> for u8 {
    fn from(led_mode: LedMode) -> Self {
        led_mode as u8
    }
}
