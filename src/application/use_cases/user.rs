use std::sync::Arc;

use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use tracing::{info, instrument};

use crate::app_error::AppResult;

#[async_trait]
pub trait UserPersistence: Send + Sync {
    async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> AppResult<()>;
}

pub trait UserCredentialsHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> AppResult<String>;
}

#[derive(Clone)]
pub struct UserUseCases {
    hasher: Arc<dyn UserCredentialsHasher>,
    persistence: Arc<dyn UserPersistence>,
}

impl UserUseCases {
    pub fn new(
        hasher: Arc<dyn UserCredentialsHasher>,
        persistence: Arc<dyn UserPersistence>,
    ) -> Self {
        Self {
            hasher,
            persistence,
        }
    }

    #[instrument(skip(self))]
    pub async fn add(&self, username: &str, email: &str, password: &SecretString) -> AppResult<()> {
        info!("Adding user...");

        let hash = &self.hasher.hash_password(password.expose_secret())?;
        self.persistence.create_user(username, email, hash).await?;

        info!("Adding user finished.");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;

    use super::*;

    struct MockUserPersistence;

    #[async_trait]
    impl UserPersistence for MockUserPersistence {
        async fn create_user(
            &self,
            username: &str,
            email: &str,
            _password_hash: &str,
        ) -> AppResult<()> {
            assert_eq!(username, "testuser");
            assert_eq!(email, "testuser@gmail.com");

            Ok(())
        }
    }

    struct MockUserCredentialsHasher;

    impl UserCredentialsHasher for MockUserCredentialsHasher {
        fn hash_password(&self, password: &str) -> AppResult<String> {
            Ok(format!("{}_hash", password))
        }
    }

    #[tokio::test]
    async fn add_user_works() {
        let user_use_cases = UserUseCases::new(
            Arc::new(MockUserCredentialsHasher),
            Arc::new(MockUserPersistence),
        );

        let result = user_use_cases
            .add("testuser", "testuser@gmail.com", &"testuser_pw".into())
            .await;

        assert!(result.is_ok());
    }
}
