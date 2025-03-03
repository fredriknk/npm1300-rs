/// Main charger enable control set
///
/// This W1S (Write-1-to-Set) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Enables the battery charger
///
/// # Read behavior
/// Returns current charger enable state:
/// - 0: Battery charging is disabled
/// - 1: Battery charging is enabled
///
/// # Register type
/// W1S (Write-1-to-Set): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerEnableSet {
    /// No effect
    NoEffect = 0,
    /// Charger is enabled
    EnableCharger = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerEnableSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::EnableCharger),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerEnableSet> for u8 {
    fn from(charger_enable_set: ChargerEnableSet) -> Self {
        charger_enable_set as u8
    }
}

/// Sets maximum charge current in cool temperature conditions
///
/// This W1S (Write-1-to-Set) register enables full charging capacity
/// when battery temperature is in the cool range.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Enables 100% charge current as defined by BCHGISETMSB:BCHGISETLSB
///
/// # Read behavior
/// Returns current charge current limit setting:
/// - 0: Cool temperature charging limited to 50% of BCHGISETMSB:BCHGISETLSB
/// - 1: Cool temperature charging allowed at 100% of BCHGISETMSB:BCHGISETLSB
///
/// # Register type
/// W1S (Write-1-to-Set): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerEnableFullCurrentChargeInCoolTempSet {
    /// No effect
    NoEffect = 0,
    /// Enable charging at full current when battery temperature is in the cool range
    EnableFullCurrentChargeInCoolTemp = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerEnableFullCurrentChargeInCoolTempSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::EnableFullCurrentChargeInCoolTemp),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerEnableFullCurrentChargeInCoolTempSet> for u8 {
    fn from(
        charger_enable_full_current_charge_in_cool_temp_set: ChargerEnableFullCurrentChargeInCoolTempSet,
    ) -> Self {
        charger_enable_full_current_charge_in_cool_temp_set as u8
    }
}

/// Main charger enable control clear
///
/// This W1C (Write-1-to-Clear) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Disables the battery charger
///
/// # Read behavior
/// Returns current charger enable state:
/// - 0: Battery charging is disabled
/// - 1: Battery charging is enabled
///
/// # Register type
/// W1C (Write-1-to-Clear): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerEnableClear {
    /// No effect
    NoEffect = 0,
    /// Charger is disabled
    DisableCharger = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerEnableClear {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::DisableCharger),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerEnableClear> for u8 {
    fn from(charger_enable_clear: ChargerEnableClear) -> Self {
        charger_enable_clear as u8
    }
}

/// Clears maximum charge current in cool temperature conditions
///
/// This W1C (Write-1-to-Clear) register enables full charging capacity
/// when battery temperature is in the cool range.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Disables the battery charger
///
/// # Read behavior
/// Returns current charge current limit setting:
/// - 0: Cool temperature charging limited to 50% of BCHGISETMSB:BCHGISETLSB
/// - 1: Cool temperature charging allowed at 100% of BCHGISETMSB:BCHGISETLSB
///
/// # Register type
/// W1C (Write-1-to-Clear): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerEnableFullCurrentChargeInCoolTempClear {
    /// No effect
    NoEffect = 0,
    /// Disable charging at full current when battery temperature is in the cool range
    DisableFullCurrentChargeInCoolTemp = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerEnableFullCurrentChargeInCoolTempClear {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::DisableFullCurrentChargeInCoolTemp),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerEnableFullCurrentChargeInCoolTempClear> for u8 {
    fn from(
        charger_enable_full_current_charge_in_cool_temp_clear: ChargerEnableFullCurrentChargeInCoolTempClear,
    ) -> Self {
        charger_enable_full_current_charge_in_cool_temp_clear as u8
    }
}

/// Battery charger disable recharge set
///
/// This W1S (Write-1-to-Set) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Disable recharging of battery once charged
///
/// # Read behavior
/// Returns current charger disable recharge state:
/// - 0: Battery recharge is enabled
/// - 1: Battery recharge is disabled
///
/// # Register type
/// W1S (Write-1-to-Set): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerDisableRechargeSet {
    /// No effect
    NoEffect = 0,
    /// Disable charger
    DisableRecharge = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerDisableRechargeSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::DisableRecharge),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerDisableRechargeSet> for u8 {
    fn from(charger_disable_recharge_set: ChargerDisableRechargeSet) -> Self {
        charger_disable_recharge_set as u8
    }
}

/// Battery Charger ignore NTC thermistor temperature limits set
///
/// This W1S (Write-1-to-Set) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect
/// - Writing 1: Charging ignores NTC thermistor temperature limits
///
/// # Read behavior
/// Returns current charger disable recharge state:
/// - 0: NTC thermistor temperature limits are not ignored
/// - 1: NTC thermistor temperature limits are ignored
///
/// # Register type
/// W1S (Write-1-to-Set): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DisableNtcSet {
    /// No effect
    NoEffect = 0,
    /// Charging ignore NTC thermistor temperature limits
    IgnoreNtc = 1,
}

// Add conversion from u8
impl TryFrom<u8> for DisableNtcSet {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::IgnoreNtc),
            _ => Err(()),
        }
    }
}
// Add conversion to u8
impl From<DisableNtcSet> for u8 {
    fn from(disable_ntc_set: DisableNtcSet) -> Self {
        disable_ntc_set as u8
    }
}

/// Battery charger disable recharge clear
///
/// This W1C (Write-1-to-Clear) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect on current state
/// - Writing 1: Enable Recharging of battery once charged
///
/// # Read behavior
/// Returns current charger disable recharge state:
/// - 0: Battery recharge is enabled
/// - 1: Battery recharge is disabled
///
/// # Register type
/// W1C (Write-1-to-Clear): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerDisableRechargeClear {
    /// No effect
    NoEffect = 0,
    /// Disable charger
    EnableRecharge = 1,
}

// Add conversion from u8
impl TryFrom<u8> for ChargerDisableRechargeClear {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::EnableRecharge),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<ChargerDisableRechargeClear> for u8 {
    fn from(charger_disable_recharge_clear: ChargerDisableRechargeClear) -> Self {
        charger_disable_recharge_clear as u8
    }
}

/// Battery Charger ignore NTC thermistor temperature limits clear
///
/// This W1C (Write-1-to-Clear) register controls the primary enable/disable
/// state of the battery charging system.
///
/// # Write behavior
/// - Writing 0: No effect
/// - Writing 1: Charging uses NTC thermistor temperature limits
///
/// # Read behavior
/// Returns current charger disable recharge state:
/// - 0: NTC thermistor temperature limits are not ignored
/// - 1: NTC thermistor temperature limits are ignored
///
/// # Register type
/// W1C (Write-1-to-Clear): Writing 1 sets the bit, writing 0 has no effect.
/// Reading returns current state.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DisableNtcClear {
    /// No effect
    NoEffect = 0,
    /// Charging uses NTC thermistor temperature limits
    UseNtc = 1,
}

// Add conversion from u8
impl TryFrom<u8> for DisableNtcClear {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::UseNtc),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<DisableNtcClear> for u8 {
    fn from(disable_ntc_clear: DisableNtcClear) -> Self {
        disable_ntc_clear as u8
    }
}

/// Battery charger termination voltages in normal temperature
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerTerminationVoltage {
    V3_50 = 0,
    V3_55 = 1,
    V3_60 = 2,
    V3_65 = 3,
    V4_00 = 4,
    V4_05 = 5,
    V4_10 = 6,
    V4_15 = 7,
    V4_20 = 8,
    V4_25 = 9,
    V4_30 = 10,
    V4_35 = 11,
    V4_40 = 12,
    V4_45 = 13,
}

// Add conversion from u8
impl From<u8> for ChargerTerminationVoltage {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::V3_50,
            1 => Self::V3_55,
            2 => Self::V3_60,
            3 => Self::V3_65,
            4 => Self::V4_00,
            5 => Self::V4_05,
            6 => Self::V4_10,
            7 => Self::V4_15,
            8 => Self::V4_20,
            9 => Self::V4_25,
            10 => Self::V4_30,
            11 => Self::V4_35,
            12 => Self::V4_40,
            13 => Self::V4_45,
            14 => Self::V3_60,
            15 => Self::V3_60,
            _ => panic!("Invalid value"),
        }
    }
}

// Add conversion to u8
impl From<ChargerTerminationVoltage> for u8 {
    fn from(value: ChargerTerminationVoltage) -> Self {
        value as u8
    }
}

/// Battery charger trickle level select
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerTrickleLevelSelect {
    V2_9 = 0,
    V2_5 = 1,
}

// Add conversion from u8
impl From<u8> for ChargerTrickleLevelSelect {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::V2_9,
            1 => Self::V2_5,
            _ => panic!("Invalid value"),
        }
    }
}

// Add conversion to u8
impl From<ChargerTrickleLevelSelect> for u8 {
    fn from(value: ChargerTrickleLevelSelect) -> Self {
        value as u8
    }
}

/// Battery charger termination current level select
/// Expressed as a percentage of the charging current
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerTerminationCurrentLevelSelect {
    /// 10% of charging current
    SEL10 = 0,
    /// 20% of charging current
    SEL20 = 1,
}

// Add conversion to u8
impl From<ChargerTerminationCurrentLevelSelect> for u8 {
    fn from(value: ChargerTerminationCurrentLevelSelect) -> Self {
        value as u8
    }
}

impl From<u8> for ChargerTerminationCurrentLevelSelect {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SEL10,
            1 => Self::SEL20,
            _ => unreachable!(),
        }
    }
}

/// Battery charger termination current level select
/// Expressed as a percentage of the charging current
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ChargerConfigDisableChargeWarm {
    /// Enable charging if battery is warm
    ENABLED = 0,
    /// Disable charging if battery is warm
    DISABLED = 1,
}

// Add conversion from u8
impl From<u8> for ChargerConfigDisableChargeWarm {
    fn from(value: u8) -> Self {
        match value {
            0 => ChargerConfigDisableChargeWarm::ENABLED,
            1 => ChargerConfigDisableChargeWarm::DISABLED,
            _ => unreachable!(),
        }
    }
}

// Add conversion to u8
impl From<ChargerConfigDisableChargeWarm> for u8 {
    fn from(value: ChargerConfigDisableChargeWarm) -> Self {
        value as u8
    }
}

/// Discharge current limit settings
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DischargeCurrentLimit {
    Low,
    High,
}

/// Temperature threshold regions for NTC measurements
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum NtcThresholdRegion {
    /// Cold temperature threshold (lowest)
    Cold,
    /// Cool temperature threshold
    Cool,
    /// Warm temperature threshold
    Warm,
    /// Hot temperature threshold (highest)
    Hot,
}

/// Die temperature threshold regions for temperature monitoring during charging
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DieTemperatureThresholdType {
    /// Stop temperature threshold
    Stop,
    /// Resume temperature threshold
    Resume,
}

#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ChargerStatus {
    /// Indicates if a battery is physically connected to the system
    ///
    /// # Returns
    /// - `true`: Battery is connected and detected
    /// - `false`: No battery is connected
    pub is_battery_present: bool,

    /// Indicates if the battery has reached full charge
    ///
    /// # Returns
    /// - `true`: Battery has reached full charge and charging has stopped
    /// - `false`: Battery is still charging or not at full capacity
    pub is_charging_complete: bool,

    /// Indicates if trickle charging mode is active
    ///
    /// Trickle charging is used when battery voltage is very low, charging at a reduced rate
    /// to safely bring the battery up to a minimum voltage level.
    ///
    /// # Returns
    /// - `true`: Trickle charging is currently active
    /// - `false`: Not in trickle charging mode
    pub is_trickle_charging: bool,

    /// Indicates if constant current charging mode is active
    ///
    /// During constant current charging, the charger maintains a steady charging current
    /// until the battery reaches a specific voltage threshold.
    ///
    /// # Returns
    /// - `true`: Constant current charging is active
    /// - `false`: Not in constant current charging mode
    pub is_constant_current_charging: bool,

    /// Indicates if constant voltage charging mode is active
    ///
    /// During constant voltage charging, the charger maintains a steady voltage while
    /// the charging current gradually decreases.
    ///
    /// # Returns
    /// - `true`: Constant voltage charging is active
    /// - `false`: Not in constant voltage charging mode
    pub is_constant_voltage_charging: bool,

    /// Indicates if the battery requires recharging
    ///
    /// This flag is set when the battery voltage drops below the recharge threshold
    /// after being previously fully charged.
    ///
    /// # Returns
    /// - `true`: Battery needs recharging
    /// - `false`: Battery does not need recharging
    pub needs_recharge: bool,

    /// Indicates if charging has been paused due to high die temperature
    ///
    /// The charger automatically pauses charging when the internal die temperature
    /// exceeds the configured DIETEMPSTOP threshold to prevent damage.
    ///
    /// # Returns
    /// - `true`: Charging is paused due to high temperature
    /// - `false`: Temperature is within acceptable range
    pub is_charging_paused_by_die_temperature: bool,

    /// Indicates if supplement mode is active
    ///
    /// Supplement mode activates when system power demand exceeds the input current limit (IBUSLIM)
    /// during charging. In this mode, the system draws additional current from the battery
    /// (up to IBATLIM) to maintain stable system voltage.
    ///
    /// # Returns
    /// - `true`: Supplement mode is active
    /// - `false`: Normal charging mode
    pub is_supplement_mode_active: bool,
}

/// Charger-FSM Error.
/// Latched error reasons.
/// Cleared with TASKS_CLEAR_CHG_ERR
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ChargerErrorReason {
    pub ntc_sensor_error: bool,
    pub vbat_sensor_error: bool,
    pub vbat_low_error: bool,
    pub vtrickle_error: bool,
    pub measurement_timeout_error: bool,
    pub charge_timeout_error: bool,
    pub trickle_timeout_error: bool,
}

/// Charger-FSM Error.
/// Latched sensor values.
/// Cleared with TASKS_CLEAR_CHG_ERR
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ChargerSensorValueDuringError {
    pub sensor_ntc_cold: bool,
    pub sensor_ntc_cool: bool,
    pub sensor_ntc_warm: bool,
    pub sensor_ntc_hot: bool,
    pub sensor_vterm: bool,
    pub sensor_recharge: bool,
    pub sensor_vtrickle: bool,
    pub sensor_vbat_low: bool,
}

/// Battery charger ibat status codes
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum IbatStatuscodes {
    IbatStatDischarge      = 4,   // 0x04
    IbatStatChargeError    = 8,   //0x08 undocumented in driver but occurs at no batt
    IbatStatChargeTrickle = 12,  // 0x0C
    IbatStatChargeCool    = 13,  // 0x0D
    IbatStatChargeNormal  = 15,  // 0x0F
}
// Add conversion from u8
impl From<u8> for IbatStatuscodes {
    fn from(value: u8) -> Self {
        defmt::info!("IbatStatuscodes: {:?}", value);
        match value {
            4 => Self::IbatStatDischarge,
            8 => Self::IbatStatChargeError,
            12 => Self::IbatStatChargeTrickle,
            13 => Self::IbatStatChargeCool,
            15 => Self::IbatStatChargeNormal,
            _ => panic!("Invalid value"),
        }
    }
}

// Add conversion to u8
impl From<IbatStatuscodes> for u8 {
    fn from(value: IbatStatuscodes) -> Self {
        value as u8
    }
}
