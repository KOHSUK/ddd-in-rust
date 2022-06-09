use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserDatabaseTrait {
    type UserId: Send + Sync;
    type UserName: Send + Sync;
    type UserData: Send + Sync;
    type UserIsPremium: Send + Sync;

    fn from_user_id(id: &Self::UserId) -> Result<String>;
    fn from_user_name(name: &Self::UserName) -> Result<String>;
    fn from_user_is_premium(is_premium: Self::UserIsPremium) -> Result<bool>;
    fn from_user_data(user: &Self::UserData) -> Result<(String, String, bool)>;

    fn to_user_id(value: &str) -> Result<Self::UserId>;
    fn to_user_name(value: &str) -> Result<Self::UserName>;
    fn to_user_is_premium(value: bool) -> Result<Self::UserIsPremium>;
    fn to_user_data(id: &str, name: &str, is_premium: bool) -> Result<Self::UserData>;

    async fn save(&self, user: &Self::UserData) -> Result<()>;
    async fn find(&self, user_name: &Self::UserName) -> Result<Self::UserData>;
    async fn find_by_id(&self, id: &Self::UserId) -> Result<Self::UserData>;
    async fn delete(&self, id: &Self::UserId) -> Result<()>;
}
