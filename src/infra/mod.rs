use crate::{
    adapters::{crypto::argon2::ArgonPasswordHasher, persistence::PostgresPersistence},
    infra::db::init_db,
};

pub mod app;
pub mod config;
pub mod db;
pub mod setup;

pub async fn postgres_persistence() -> anyhow::Result<PostgresPersistence> {
    let pool = init_db().await?;
    let persistence = PostgresPersistence::new(pool);
    Ok(persistence)
}

pub fn argon2_password_hasher() -> ArgonPasswordHasher {
    ArgonPasswordHasher::default()
}
