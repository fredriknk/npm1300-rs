use crate::{common::Task, field_sets::Shphldstatus, Shphldtim};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Enter hibernation mode
    pub async fn enter_hibernate_mode(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .taskenterhibernate()
            .dispatch_async(|command| command.set_taskenterhibernate(Task::Trigger))
            .await
    }

    /// Enter ship mode
    pub async fn enter_ship_mode(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .taskentershipmode()
            .dispatch_async(|command| command.set_taskentershipmode(Task::Trigger))
            .await
    }

    /// Request a reset of the ship hold configuration
    pub async fn reset_ship_hold_config(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .taskresetcfg()
            .dispatch_async(|command| command.set_taskshphldrstconfig(Task::Trigger))
            .await
    }

    /// Set the ship hold press timer
    pub async fn set_ship_hold_press_timer(
        &mut self,
        time: Shphldtim,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .shphldconfig()
            .write_async(|reg| reg.set_shphldtim(time))
            .await?;

        // Load the new configuration
        self.device
            .ship()
            .taskshphldcfgstrobe()
            .dispatch_async(|command| command.set_taskshphldconfigstrobe(Task::Trigger))
            .await
    }

    /// Get the ship hold status
    ///
    /// This will return the status of the SHPHLD pin.
    pub async fn get_ship_hold_status(
        &mut self,
    ) -> Result<Shphldstatus, crate::NPM1300Error<I2c::Error>> {
        self.device.ship().shphldstatus().read_async().await
    }

    /// Enable the long press reset
    pub async fn enable_long_press_reset(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .lpresetconfig()
            .modify_async(|reg| reg.set_longtimresetdis(crate::Longtimresetdis::Enabled))
            .await?;

        // Load the new configuration
        self.device
            .ship()
            .taskshphldcfgstrobe()
            .dispatch_async(|command| command.set_taskshphldconfigstrobe(Task::Trigger))
            .await
    }

    /// Disable the long press reset
    pub async fn disable_long_press_reset(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .lpresetconfig()
            .modify_async(|reg| reg.set_longtimresetdis(crate::Longtimresetdis::Disabled))
            .await?;

        // Load the new configuration
        self.device
            .ship()
            .taskshphldcfgstrobe()
            .dispatch_async(|command| command.set_taskshphldconfigstrobe(Task::Trigger))
            .await
    }

    /// Use the ship hold button only
    pub async fn use_ship_hold_button_only(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .lpresetconfig()
            .modify_async(|reg| reg.set_longtimtwobuttonsel(crate::Longtimtwobuttonsel::Shphld))
            .await?;

        // Load the new configuration
        self.device
            .ship()
            .taskshphldcfgstrobe()
            .dispatch_async(|command| command.set_taskshphldconfigstrobe(Task::Trigger))
            .await
    }

    /// Use the ship hold button and GPIO0 to perform long press reset
    pub async fn use_ship_hold_button_and_gpio0(
        &mut self,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ship()
            .lpresetconfig()
            .modify_async(|reg| {
                reg.set_longtimtwobuttonsel(crate::Longtimtwobuttonsel::Shphldgpio0)
            })
            .await?;

        // Load the new configuration
        self.device
            .ship()
            .taskshphldcfgstrobe()
            .dispatch_async(|command| command.set_taskshphldconfigstrobe(Task::Trigger))
            .await
    }
}
