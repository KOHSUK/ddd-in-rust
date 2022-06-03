use anyhow::{Result, anyhow};

use super::{UserId, UserName};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Result<User> {
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