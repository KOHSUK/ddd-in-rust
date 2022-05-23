use std::{collections::HashMap};

use crate::interface::repository::user_database::{UserId, UserName, UserData, UserDatabase};

use anyhow::{ Result, anyhow, Ok };
use async_trait::async_trait;
use tokio::sync::Mutex;
use sqlx::types::Uuid;
use once_cell::sync::Lazy;

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
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
        }
    }
}

type UserTable = HashMap<String, UserRow>;

pub struct InMemoryUserRepository {
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl UserDatabase for InMemoryUserRepository {
    async fn save(&self, user_name: &UserName) -> Result<()> {
        let row = UserRow::new(user_name);
        let mut table = STATIC_USER_TABLE.lock().await;
        table.insert(row.clone().id, row);
        table.iter().for_each(|r| { dbg!(r.1); });

        Ok(())
    }

    async fn update(&self, user: &UserData) -> Result<()> {
        let mut table = STATIC_USER_TABLE.lock().await;
        let row = UserRow { id: user.clone().0.to_string(), name: user.clone().1 };
        table.insert(row.clone().id, row);

        Ok(())
    }

    async fn find(&self, user_name: &UserName) -> Result<UserData> {
        let table = STATIC_USER_TABLE.lock().await;
        table.iter().find(|row| row.1.name == *user_name)
            .map(|row| {
                let row = row.1.clone();
                (Uuid::parse_str(&row.id).unwrap(), row.name)
            })
            .ok_or_else(|| anyhow!("User not found"))
    }

    async fn delete(&self, user_id: &UserId) -> Result<()> {
        let mut table = STATIC_USER_TABLE.lock().await;
        table.remove(&user_id.to_string());

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<UserData> {
        let table = STATIC_USER_TABLE.lock().await;
        table.get(&id.to_string())
            .map(|row| {
                let id = &row.id;
                let id = Uuid::parse_str(id).unwrap();
                let name = row.name.to_owned();
                (id, name)
            })
            .ok_or_else(|| anyhow!("User not found"))
    }
}