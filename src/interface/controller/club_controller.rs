use std::sync::{Arc, Mutex};

use anyhow::Result;
use sqlx::postgres;

use crate::{
    application::club::{ClubCreateCommand, ClubCreateService, ClubJoinCommand, ClubJoinService},
    domain::model::club::{factory::ClubFactory, service::ClubService},
    infrastructure::database::{
        club::PostgresClubDatabase, shared::DATABASE_CONFIG, user::PostgresUserDatabase,
    },
    interface::repository::{club::ClubRepository, user::UserRepository},
};

pub struct ClubController {
    club_create_service: ClubCreateService,
    club_join_service: ClubJoinService,
}

pub struct PostClubArgs {
    pub user_id: String,
    pub name: String,
}

pub struct PostMemberArgs {
    pub user_id: String,
    pub club_id: String,
}

impl ClubController {
    pub async fn new() -> Result<Self> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&DATABASE_CONFIG.database_url())
            .await?;
        let pool = Arc::new(pool);

        let pgpool = Arc::clone(&pool);
        // repository
        let club_database = Box::new(PostgresClubDatabase::new(pgpool)?);
        let club_repository = ClubRepository::new(club_database).await?;
        let club_repository = Arc::new(Mutex::new(club_repository));

        // factory
        let club_factory = Arc::new(ClubFactory::new());

        // domain service
        let club_repo = Arc::clone(&club_repository);
        let club_service = Arc::new(ClubService::new(club_repo));

        let pgpool = Arc::clone(&pool);
        // user repository
        let user_database = PostgresUserDatabase::new(pgpool)?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let user_repository = Arc::new(user_repository);

        let club_repo = Arc::clone(&club_repository);
        let club_fac = Arc::clone(&club_factory);
        let club_ser = Arc::clone(&club_service);
        let user_repo = Arc::clone(&user_repository);
        let club_create_service = ClubCreateService::new(club_repo, club_fac, club_ser, user_repo);

        let club_repo = Arc::clone(&club_repository);
        let club_fac = Arc::clone(&club_factory);
        let club_ser = Arc::clone(&club_service);
        let user_repo = Arc::clone(&user_repository);
        let club_join_service = ClubJoinService::new(club_repo, club_fac, club_ser, user_repo);

        Ok(Self {
            club_create_service,
            club_join_service,
        })
    }

    pub async fn post_club(&self, args: PostClubArgs) -> Result<()> {
        let command = ClubCreateCommand::new(&args.user_id, &args.name);
        self.club_create_service.handle(command).await
    }

    pub async fn post_member(&self, args: PostMemberArgs) -> Result<()> {
        let command = ClubJoinCommand::new(&args.user_id, &args.club_id);
        self.club_join_service.handle(command).await
    }
}
