use crate::domain::model::user::{
    entity::{User, UserId, UserIsPremium, UserName},
    repository::UserRepositoryTrait,
};
use crate::interface::repository::user::UserDatabaseTrait;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UserDatabaseTraitWrapper {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>>;
    async fn delete(&self, id: &UserId) -> Result<()>;
}

#[async_trait]
impl<D: UserDatabaseTrait + Send + Sync> UserDatabaseTraitWrapper for D {
    async fn save(&self, user: &User) -> Result<()> {
        let user = D::to_user_data(
            &user.get_id().to_string(),
            &user.get_name().to_string(),
            user.get_is_premium().to_inner(),
        )?;
        self.save(&user).await
    }

    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>> {
        let user_name = D::to_user_name(&user_name.to_string())?;
        let user = self.find(&user_name).await?;
        let user = D::from_user_data(&user)?;

        let user_id = UserId::new(&user.0)?;
        let user_name = UserName::new(&user.1)?;
        let is_premium = UserIsPremium::new(user.2);
        let user = User::new(user_id, user_name, is_premium)?;

        Ok(Some(user))
    }

    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>> {
        let user_id = D::to_user_id(&user_id.to_string())?;
        let user = self.find_by_id(&user_id).await?;
        let user = D::from_user_data(&user)?;

        let user_id = UserId::new(&user.0)?;
        let user_name = UserName::new(&user.1)?;
        let is_premium = UserIsPremium::new(user.2);
        let user = User::new(user_id, user_name, is_premium)?;

        Ok(Some(user))
    }

    async fn delete(&self, user_id: &UserId) -> Result<()> {
        let user_id = D::to_user_id(&user_id.to_string())?;
        self.delete(&user_id).await
    }
}

pub struct UserRepository {
    database: Box<dyn UserDatabaseTraitWrapper + Sync + Send>,
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        self.database.save(user).await
    }

    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>> {
        self.database.find_by_name(user_name).await
    }

    async fn delete(&self, id: &UserId) -> Result<()> {
        self.database.delete(id).await
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        self.database.find_by_id(id).await
    }
}

impl UserRepository {
    pub async fn new(
        database: Box<dyn UserDatabaseTraitWrapper + Sync + Send>,
    ) -> anyhow::Result<Self> {
        let repo = Self { database };

        Ok(repo)
    }
}
