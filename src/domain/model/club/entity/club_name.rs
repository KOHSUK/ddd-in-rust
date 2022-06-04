use anyhow::{Result};
use validator::{Validate};

#[derive(Debug, Clone, Validate)]
pub struct ClubName {
    #[validate(length(min = 3))]
    value: String,
}

impl ClubName {
    pub fn new(value: &str) -> Result<Self> {
        let data = Self {
            value: value.to_string()
        };
        data.validate()?;
        Ok(data)
    }

    pub fn to_str(&self) -> &str {
        &self.value
    }
}