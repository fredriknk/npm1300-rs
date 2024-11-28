use crate::{common::Task, Vbatautoenable};
use libm::logf;

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Measure VBAT
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VBAT voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VBAT measurement result
    pub async fn measure_vbat(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VBAT measurement...");
        self.device
            .adc()
            .taskvbatmeasure()
            .dispatch_async(|command| command.set_taskvbatmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcvbatresultmsb()
            .read_async()
            .await?
            .vbatresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .vbatresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVBAT, the full scale voltage for measuring VBAT.
        // 1023.0 is the maximum value for the 10 bit ADC.
        let result = (result as f32 / 1023.0) * 5.0;

        Ok(result)
    }
    /// Measure NTC
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured NTC resistance in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the NTC measurement result
    //TODO: test this function
    pub async fn measure_ntc(
        &mut self,
        ntc_beta: f32,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering NTC measurement...");
        self.device
            .adc()
            .taskntcmeasure()
            .dispatch_async(|command| command.set_taskntcmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcntcresultmsb()
            .read_async()
            .await?
            .ntcresultmsb();
        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .ntcresultlsb();
        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;
        // Convert result to f32
        // The temperature is returned in degrees Celsius
        let result = 1.0
            / ((1.0 / 298.15) - (1.0 / ntc_beta) * logf((1024.0 / result as f32) - 1.0))
            - 273.15;
        Ok(result)
    }

    /// Measure die temperature
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured die temperature in degrees Celsius
    /// * `Err(NPM1300Error)` - An error occurred while reading the die temperature measurement result
    pub async fn measure_die_temperature(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering die temperature measurement...");
        self.device
            .adc()
            .tasktempmeasure()
            .dispatch_async(|command| command.set_tasktempmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adctempresultmsb()
            .read_async()
            .await?
            .tempresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .tempresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // The temperature is returned in degrees Celsius
        let result = 394.67 - 0.7926 * result as f32;

        Ok(result)
    }

    /// Measure VSYS
    ///
    /// # Returns
    ///
    /// * `Ok(f32)` - The measured VSYS voltage
    /// * `Err(NPM1300Error)` - An error occurred while reading the VSYS measurement result
    pub async fn measure_vsys(
        &mut self,
        // ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
    ) -> Result<f32, crate::NPM1300Error<I2c::Error>> {
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Triggering VSYS measurement...");
        self.device
            .adc()
            .taskvsysmeasure()
            .dispatch_async(|command| command.set_taskvsysmeasure(Task::Trigger))
            .await?;

        // Wait for measurement to complete
        #[cfg(feature = "defmt-03")]
        defmt::debug!("Waiting for measurement to complete...");
        self.delay.delay_us(250).await;

        // Read measurement result
        let msb = self
            .device
            .adc()
            .adcvsysresultmsb()
            .read_async()
            .await?
            .vsysresultmsb();

        let lsb = self
            .device
            .adc()
            .adcgp_0_resultlsbs()
            .read_async()
            .await?
            .vsysresultlsb();

        // Convert result to u16
        let result = ((msb as u16) << 2) | (lsb & 0x03) as u16;

        // Convert result to f32
        // 5.0 is VFSVSYS, the full scale voltage for measuring VSYS.
        // 1023.0 is the maximum value for the 10 bit ADC.
        let result = (result as f32 / 1023.0) * 5.0;

        Ok(result)
    }

    /// Configure auto VBAT measurement
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, enable auto VBAT measurement every 1 second, otherwise single measurement when triggered
    pub async fn configure_auto_vbat_measurement(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .adc()
            .adcconfig()
            .write_async(|reg| {
                reg.set_vbatautoenable(if enable {
                    Vbatautoenable::Autoenable
                } else {
                    Vbatautoenable::Noauto
                })
            })
            .await
    }
}
