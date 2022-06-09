use anyhow::Result;
use validator::Validate;

use super::{UserId, UserIsPremium, UserName};

#[derive(Debug, Clone, Validate)]
pub struct User {
    #[validate]
    id: UserId,
    #[validate]
    name: UserName,
    is_premium: UserIsPremium,
}

impl User {
    pub fn new(id: UserId, name: UserName, is_premium: UserIsPremium) -> Result<Self> {
        let data = Self {
            id,
            name,
            is_premium,
        };
        data.validate()?;
        Ok(data)
    }

    pub fn get_name(&self) -> &UserName {
        &self.name
    }

    pub fn get_id(&self) -> &UserId {
        &self.id
    }

    pub fn get_is_premium(&self) -> &UserIsPremium {
        &self.is_premium
    }

    pub fn change_name(&mut self, name: UserName) -> Result<()> {
        self.name = name;
        self.validate()?;

        Ok(())
    }

    pub fn upgrade(&mut self) -> Result<()> {
        self.is_premium = UserIsPremium::new(true);

        Ok(())
    }

    pub fn downgrade(&mut self) -> Result<()> {
        self.is_premium = UserIsPremium::new(false);

        Ok(())
    }
}
