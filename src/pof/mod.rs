use crate::{Pofena, Pofwarnpolarity, VsysThreshold};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Enable or disable power-failure detection
    ///
    /// # Arguments
    ///
    /// * `enable` - true to enable power-failure detection, false to disable it
    pub async fn enable_power_failure_detection(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .pof()
            .pofconfig()
            .modify_async(|reg| reg.set_pofena(if enable { Pofena::Enabled } else { Pofena::Off }))
            .await
    }

    /// Check if power failure detection is enabled
    ///
    /// # Returns
    ///
    /// Returns `true` if power failure detection is enabled, `false` if disabled, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn is_power_failure_detection_enabled(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        Ok(self.device.pof().pofconfig().read_async().await?.pofena() == Pofena::Enabled)
    }

    pub async fn set_power_failure_warning_gpio_polarity(
        &mut self,
        polarity: Pofwarnpolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .pof()
            .pofconfig()
            .modify_async(|reg| reg.set_pofwarnpolarity(polarity))
            .await
    }

    /// Get the polarity of the power failure warning GPIO
    ///
    /// This function does not check if a GPIO is configured as a power failure warning
    /// GPIO. It is the caller's responsibility to ensure that the GPIO is configured
    /// correctly.
    ///
    /// # Returns
    ///
    /// Returns the current polarity of the power failure warning GPIO, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn get_power_failure_warning_gpio_polarity(
        &mut self,
    ) -> Result<Pofwarnpolarity, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .pof()
            .pofconfig()
            .read_async()
            .await?
            .pofwarnpolarity())
    }

    /// Set the VSYS (System Voltage) threshold for power failure detection
    ///
    /// # Arguments
    ///
    /// * `threshold` - The VSYS threshold voltage level to trigger power failure detection
    ///
    /// # Safety
    ///
    /// 1. The VSYS threshold must be lower than the current VSYS voltage, otherwise it will
    ///    immediately trigger a power failure event and device reset. This is checked by the driver.
    ///
    /// 2. The VSYS threshold must be higher than the battery undervoltage protection level to
    ///    prevent the protection circuit from triggering. This is NOT checked by the driver
    ///    and must be ensured by the caller.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the threshold was set successfully, or `Err(NPM1300Error::InvalidPofVsysThreshold)`
    /// if the requested threshold is higher than the current VSYS voltage, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn set_vsys_threshold(
        &mut self,
        threshold: VsysThreshold,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Check if the threshold is safe
        let vsys = self.measure_vsys().await?;
        let threshold_voltage = match threshold {
            VsysThreshold::V26 => 2.6,
            VsysThreshold::V27 => 2.7,
            VsysThreshold::V28 => 2.8,
            VsysThreshold::V29 => 2.9,
            VsysThreshold::V30 => 3.0,
            VsysThreshold::V31 => 3.1,
            VsysThreshold::V32 => 3.2,
            VsysThreshold::V33 => 3.3,
            VsysThreshold::V34 => 3.4,
            VsysThreshold::V35 => 3.5,
            VsysThreshold::Unused10 => 2.8,
            VsysThreshold::Unused11 => 2.8,
            VsysThreshold::Unused12 => 2.8,
            VsysThreshold::Unused13 => 2.8,
            VsysThreshold::Unused14 => 2.8,
            VsysThreshold::Unused15 => 2.8,
        };
        if vsys < threshold_voltage {
            return Err(crate::NPM1300Error::InvalidPofVsysThreshold);
        }

        // Configure the threshold
        self.device
            .pof()
            .pofconfig()
            .modify_async(|reg| reg.set_pofvsysthreshsel(threshold))
            .await
    }

    /// Get the VSYS (System Voltage) threshold for power failure detection
    ///
    /// # Returns
    ///
    /// Returns the current VSYS threshold voltage level, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn get_vsys_threshold(
        &mut self,
    ) -> Result<VsysThreshold, crate::NPM1300Error<I2c::Error>> {
        // We trust the PMIC to return a valid value
        // We can safely unwrap here because we know the register is valid
        Ok(self
            .device
            .pof()
            .pofconfig()
            .read_async()
            .await?
            .pofvsysthreshsel())
    }
}
