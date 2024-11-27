/// Input current limit for VBUS
#[derive(Copy, Clone, Debug)]
pub enum VbusInCurrentLimit {
    // 500 mA
    MA500 = 0,
    // 100 mA
    MA100 = 1,
    // 200 mA
    MA200 = 2,
    // 300 mA
    MA300 = 3,
    // 400 mA
    MA400 = 4,
    // 500 mA (alternate)
    MA500Alt = 5,
    // 600 mA
    MA600 = 6,
    // 700 mA
    MA700 = 7,
    // 800 mA
    MA800 = 8,
    // 900 mA
    MA900 = 9,
    // 1000 mA
    MA1000 = 10,
    // 1100 mA
    MA1100 = 11,
    // 1200 mA
    MA1200 = 12,
    // 1300 mA
    MA1300 = 13,
    // 1400 mA
    MA1400 = 14,
    // 1500 mA
    MA1500 = 15,
}

// Add conversion from u8
impl From<u8> for VbusInCurrentLimit {
    fn from(value: u8) -> Self {
        match value {
            0 => VbusInCurrentLimit::MA500,
            1 => VbusInCurrentLimit::MA100,
            2 => VbusInCurrentLimit::MA200,
            3 => VbusInCurrentLimit::MA300,
            4 => VbusInCurrentLimit::MA400,
            5 => VbusInCurrentLimit::MA500Alt,
            6 => VbusInCurrentLimit::MA600,
            7 => VbusInCurrentLimit::MA700,
            8 => VbusInCurrentLimit::MA800,
            9 => VbusInCurrentLimit::MA900,
            10 => VbusInCurrentLimit::MA1000,
            11 => VbusInCurrentLimit::MA1100,
            12 => VbusInCurrentLimit::MA1200,
            13 => VbusInCurrentLimit::MA1300,
            14 => VbusInCurrentLimit::MA1400,
            15 => VbusInCurrentLimit::MA1500,
            _ => unreachable!(),
        }
    }
}

// Add conversion to u8
impl From<VbusInCurrentLimit> for u8 {
    fn from(value: VbusInCurrentLimit) -> Self {
        value as u8
    }
}

/// Input current limit for VBUS
#[derive(Copy, Clone, Debug)]
pub enum VbusInCcCmp {
    /// No connection
    NoConnection = 0,
    /// Default USB 100/500mA
    DefaultUsb = 1,
    /// 1.5A High Power
    MA1500HighPower = 2,
    /// 3A High Power
    MA3000HighPower = 3,
}

// Add conversion from u8
impl From<u8> for VbusInCcCmp {
    fn from(value: u8) -> Self {
        match value {
            0 => VbusInCcCmp::NoConnection,
            1 => VbusInCcCmp::DefaultUsb,
            2 => VbusInCcCmp::MA1500HighPower,
            3 => VbusInCcCmp::MA3000HighPower,
            _ => unreachable!(),
        }
    }
}

// Add conversion to u8
impl From<VbusInCcCmp> for u8 {
    fn from(value: VbusInCcCmp) -> Self {
        value as u8
    }
}
