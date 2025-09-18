use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::use_cases::{
    app_error::{AppError, AppResult},
    user::UserCredentialsHasher,
};

#[derive(Default)]
pub struct ArgonPasswordHasher {
    hasher: Argon2<'static>,
}

impl UserCredentialsHasher for ArgonPasswordHasher {
    fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .hasher
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Internal("Password hashing failed.".into()))?
            .to_string();

        Ok(hash)
    }
}
