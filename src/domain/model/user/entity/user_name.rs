use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn new(_value: &str) -> Result<UserName> {
        if _value.is_empty() {
            return Err(anyhow!("User name cannot be empty."));
        }
        if _value.len() < 3 {
            return Err(anyhow!("The length of a user name must be greater than 3"));
        }
        Ok(UserName {
            value: _value.to_string()
        })
    }

    pub fn to_str(&self) -> &str {
        &self.value
    }
}