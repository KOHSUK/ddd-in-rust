use std::fmt::Display;

use anyhow::Result;
use validator::Validate;

#[derive(Debug, Clone, Validate, PartialEq, Eq)]
pub struct ClubName {
    #[validate(length(min = 3))]
    value: String,
}

impl ClubName {
    pub fn new(value: &str) -> Result<Self> {
        let data = Self {
            value: value.to_string(),
        };
        data.validate()?;
        Ok(data)
    }
}

impl Display for ClubName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
