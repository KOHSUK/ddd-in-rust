use std::collections::HashMap;

use crate::interface::repository::user::UserDatabaseTrait;

use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static STATIC_USER_TABLE: Lazy<Mutex<UserTable>> = Lazy::new(|| {
    let table = UserTable::new();
    Mutex::new(table)
});

#[derive(Clone, Debug)]
struct UserRow {
    id: String,
    name: String,
}

impl UserRow {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

type UserTable = HashMap<String, UserRow>;

pub struct InMemoryUserDatabase {}

impl InMemoryUserDatabase {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserDatabaseTrait for InMemoryUserDatabase {
    type UserId = String;
    type UserName = String;
    type UserData = (Self::UserId, Self::UserName);

    fn from_user_id(id: &Self::UserId) -> Result<String> {
        Ok(id.to_string())
    }
    fn from_user_name(name: &Self::UserName) -> Result<String> {
        Ok(name.to_owned())
    }
    fn from_user_data(user: &Self::UserData) -> Result<(String, String)> {
        Ok((user.0.to_owned(), user.1.to_owned()))
    }

    fn to_user_id(value: &str) -> Result<Self::UserId> {
        Ok(value.to_string())
    }
    fn to_user_name(value: &str) -> Result<Self::UserName> {
        Ok(value.to_string())
    }
    fn to_user_data(id: &str, name: &str) -> Result<Self::UserData> {
        Ok((id.to_string(), name.to_string()))
    }

    async fn save(&self, user: &Self::UserData) -> Result<()> {
        let row = UserRow::new(&user.0, &user.1);
        let mut table = STATIC_USER_TABLE.lock().await;
        table.insert(row.clone().id, row);

        Ok(())
    }

    async fn find(&self, user_name: &Self::UserName) -> Result<Self::UserData> {
        let table = STATIC_USER_TABLE.lock().await;
        table
            .iter()
            .find(|row| row.1.name == *user_name)
            .map(|row| {
                let row = row.1.clone();
                (row.id, row.name)
            })
            .ok_or_else(|| anyhow!("User not found"))
    }

    async fn delete(&self, user_id: &Self::UserId) -> Result<()> {
        let mut table = STATIC_USER_TABLE.lock().await;
        table.remove(user_id);

        Ok(())
    }

    async fn find_by_id(&self, id: &Self::UserId) -> Result<Self::UserData> {
        let table = STATIC_USER_TABLE.lock().await;
        table
            .get(id)
            .map(|row| (row.id.to_owned(), row.name.to_owned()))
            .ok_or_else(|| anyhow!("User not found"))
    }
}
