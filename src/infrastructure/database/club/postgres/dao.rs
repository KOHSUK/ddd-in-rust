use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{self, types::Uuid, Pool, Postgres};
use std::sync::Mutex;

use crate::interface::repository::club::{
    ClubDatabaseTrait, PrimitiveId, PrimitiveMembers, PrimitiveName, PrimitiveOwner,
};

pub struct PostgresClubDatabase {
    pool: Arc<Pool<Postgres>>,
}

#[async_trait]
impl ClubDatabaseTrait for PostgresClubDatabase {
    type ClubId = Uuid;
    type ClubName = String;
    type ClubMembers = Vec<String>;
    type ClubOwner = Uuid;
    type ClubData = (
        Self::ClubId,
        Self::ClubName,
        Self::ClubOwner,
        Self::ClubMembers,
    );

    fn from_club_id(id: &Self::ClubId) -> Result<PrimitiveId> {
        Ok(id.to_string())
    }
    fn from_club_name(name: &Self::ClubName) -> Result<PrimitiveName> {
        Ok(name.to_owned())
    }
    fn from_club_owner(owner: &Self::ClubOwner) -> Result<PrimitiveOwner> {
        Ok(owner.to_string())
    }
    fn from_club_members(members: &Self::ClubMembers) -> Result<PrimitiveMembers> {
        Ok(members.to_owned())
    }
    fn from_club_data(
        club: &Self::ClubData,
    ) -> Result<(PrimitiveId, PrimitiveName, PrimitiveOwner, PrimitiveMembers)> {
        let club = (
            club.0.to_string(),
            club.1.to_owned(),
            club.2.to_string(),
            club.3.to_owned(),
        );
        Ok(club)
    }

    fn to_club_id(value: &PrimitiveId) -> Result<Self::ClubId> {
        Uuid::parse_str(value).map_err(|e| anyhow!(e.to_string()))
    }
    fn to_club_name(value: &PrimitiveName) -> Result<Self::ClubName> {
        Ok(value.to_owned())
    }
    fn to_club_owner(value: &PrimitiveOwner) -> Result<Self::ClubOwner> {
        Uuid::parse_str(value).map_err(|e| anyhow!(e.to_string()))
    }
    fn to_club_members(members: &PrimitiveMembers) -> Result<Self::ClubMembers> {
        Ok(members.to_owned())
    }
    fn to_club_data(
        id: &PrimitiveId,
        name: &PrimitiveName,
        owner_id: &PrimitiveOwner,
        members: &PrimitiveMembers,
    ) -> Result<Self::ClubData> {
        let id = Self::to_club_id(id)?;
        let name = Self::to_club_name(name)?;
        let owner = Self::to_club_owner(owner_id)?;
        let members = Self::to_club_members(members)?;

        Ok((id, name, owner, members))
    }

    async fn save(&self, club: &Self::ClubData) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        let id = club.0;
        let name = club.1.to_string();
        let owner = club.2;
        let members = &club.3;

        sqlx::query(
            "
insert into public.club (id, name, owner) values ($1, $2, $3)
on conflict on constraint club_id_key
do
update set name = $2, owner = $3;
            ",
        )
        .bind(id.to_owned())
        .bind(name)
        .bind(owner)
        .execute(&mut *conn)
        .await?;

        let mut conn = self.pool.acquire().await?;

        for member in members {
            let member = Uuid::parse_str(member)?;
            sqlx::query(
                "
insert into public.club_members (club_id, user_id) 
select $1, $2 where not exists (select 1 from public.club_members where club_id = $1 and user_id = $2);
                "
            )
            .bind(id)
            .bind(member)
            .execute(&mut *conn)
            .await?;
        }

        Ok(())
    }

    async fn find_by_name(&self, club_name: &Self::ClubName) -> Result<Self::ClubData> {
        let mut conn = self.pool.acquire().await?;
        type Id = Uuid;
        type Name = String;
        type Owner = Uuid;

        let data = sqlx::query_as::<_, (Id, Name, Owner)>(
            "select id, name, owner from club where name = $1",
        )
        .bind(club_name)
        .fetch_one(&mut *conn)
        .await?;

        let club_id = data.0;

        #[derive(sqlx::FromRow)]
        struct Response(Uuid);

        let members =
            sqlx::query_as::<_, Response>("select user_id from club_members where club_id = $1")
                .bind(club_id)
                .fetch_all(&mut *conn)
                .await?;
        let members = members
            .iter()
            .map(|m| m.0.to_string())
            .collect::<Vec<String>>();

        Ok((data.0, data.1, data.2, members))
    }

    async fn find_by_id(&self, id: &Self::ClubId) -> Result<Self::ClubData> {
        let mut conn = self.pool.acquire().await?;
        type Id = Uuid;
        type Name = String;
        type Owner = Uuid;

        let club_id = id;

        let data = sqlx::query_as::<_, (Id, Name, Owner)>(
            "select id, name, owner from club where id = $1",
        )
        .bind(club_id)
        .fetch_one(&mut *conn)
        .await?;

        #[derive(sqlx::FromRow)]
        struct Response(Uuid);

        let members =
            sqlx::query_as::<_, Response>("select user_id from club_members where club_id = $1")
                .bind(club_id)
                .fetch_all(&mut *conn)
                .await?;
        let members = members
            .iter()
            .map(|m| m.0.to_string())
            .collect::<Vec<String>>();

        Ok((data.0, data.1, data.2, members))
    }

    async fn find_all(&self) -> Result<Vec<Self::ClubData>> {
        let mut conn = self.pool.acquire().await?;
        type Id = Uuid;
        type Name = String;
        type Owner = Uuid;
        type Member = Uuid;

        let data = sqlx::query_as::<_, (Id, Name, Owner, Member)>(
            "
            select club.id, club.name, club.owner, public.user.id from club
            left outer join club_members
            on club.id = club_members.club_id
            left outer join public.user
            on club_members.user_id = public.user.id;
            ",
        )
        .fetch_all(&mut conn)
        .await?;

        let mut clubs: HashMap<String, Mutex<Self::ClubData>> = HashMap::new();
        data.iter().for_each(|c| {
            if let Some(club) = clubs.get(&c.0.to_string()) {
                let mut club = club.lock().unwrap();
                club.3.push(c.3.to_string());
            } else {
                let club = (c.0, c.1.to_string(), c.2, vec![c.3.to_string()]);
                clubs.insert(c.0.to_string(), Mutex::new(club));
            }
        });

        Ok(clubs
            .into_values()
            .map(|a| a.into_inner().unwrap())
            .collect::<Vec<Self::ClubData>>())
    }
}

impl PostgresClubDatabase {
    pub fn new(pool: Arc<Pool<Postgres>>) -> anyhow::Result<Self> {
        Ok(Self { pool })
    }
}
