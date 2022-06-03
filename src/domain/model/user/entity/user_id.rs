use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct UserId {
    value: String,
}

impl UserId {
    pub fn new(_value: &str) -> Result<UserId> {
        if _value.is_empty() {
            return Err(anyhow!("The ID cannot be empty"));
        }
        Ok(UserId {
            value: _value.to_string()
        })
    }

    pub fn to_str(&self) -> &str {
        &self.value
    }
}