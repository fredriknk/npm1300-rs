/// Task
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Task {
    /// Has no effect
    NoEffect = 0,
    /// Trigger task
    Trigger = 1,
}

// Add conversion from u8
impl TryFrom<u8> for Task {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoEffect),
            1 => Ok(Self::Trigger),
            _ => Err(()),
        }
    }
}

// Add conversion to u8
impl From<Task> for u8 {
    fn from(task: Task) -> Self {
        task as u8
    }
}
