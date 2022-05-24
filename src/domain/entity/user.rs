use rand::prelude::*;
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

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Result<User> {
        let id: u8 = random();
        let id = UserId::new(&id.to_string())?;
        let user = User {
            id,
            name,
        };
        Ok(user)
    }

    pub fn new_with_id(id: UserId, name: UserName) -> Result<User> {
        let user = User {
            id,
            name
        };
        Ok(user)
    }

    pub fn get_name(&self) -> UserName {
        self.name.clone()
    }

    pub fn get_id(&self) -> UserId {
        self.id.clone()
    }

    pub fn change_name(&mut self, name: UserName) -> Result<()> {
        if name.to_str().is_empty() {
            return Err(anyhow!("Name cannot be empty."));
        }

        self.name = name;

        Ok(())
    }
}