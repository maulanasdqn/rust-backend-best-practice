use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone, Debug)]
pub struct PasswordHashService {
    argon2: Argon2<'static>,
}

impl PasswordHashService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {e}"))?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> anyhow::Result<bool> {
        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {e}"))?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl Default for PasswordHashService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let service = PasswordHashService::new();
        let password = "secure_password_123";

        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_verify_wrong_password() {
        let service = PasswordHashService::new();
        let password = "secure_password_123";
        let wrong_password = "wrong_password";

        let hash = service.hash_password(password).unwrap();
        assert!(!service.verify_password(wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let service = PasswordHashService::new();
        let password = "secure_password_123";

        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();

        assert_ne!(hash1, hash2);

        assert!(service.verify_password(password, &hash1).unwrap());
        assert!(service.verify_password(password, &hash2).unwrap());
    }
}
