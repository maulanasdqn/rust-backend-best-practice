use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::domain::Claims;

/// Service for generating and verifying JWT tokens
#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiry_minutes: i64,
    refresh_token_expiry_minutes: i64,
}

impl std::fmt::Debug for JwtService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtService")
            .field("encoding_key", &"<secret>")
            .field("decoding_key", &"<secret>")
            .field(
                "access_token_expiry_minutes",
                &self.access_token_expiry_minutes,
            )
            .field(
                "refresh_token_expiry_minutes",
                &self.refresh_token_expiry_minutes,
            )
            .finish()
    }
}

impl JwtService {
    /// Creates a new `JwtService` with the provided secret and expiry settings
    ///
    /// # Arguments
    /// * `jwt_secret` - The secret key used for signing tokens
    /// * `access_token_expiry_minutes` - Expiry time for access tokens (default: 15 minutes)
    /// * `refresh_token_expiry_minutes` - Expiry time for refresh tokens (default: 10080 minutes = 7 days)
    pub fn new(
        jwt_secret: &str,
        access_token_expiry_minutes: Option<i64>,
        refresh_token_expiry_minutes: Option<i64>,
    ) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            access_token_expiry_minutes: access_token_expiry_minutes.unwrap_or(15),
            refresh_token_expiry_minutes: refresh_token_expiry_minutes.unwrap_or(10080), // 7 days
        }
    }

    /// Generates a new access token for a user
    ///
    /// # Arguments
    /// * `user_id` - The user's UUID
    /// * `email` - The user's email address
    ///
    /// # Returns
    /// A signed JWT access token as a string
    pub fn generate_access_token(&self, user_id: Uuid, email: String) -> anyhow::Result<String> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::minutes(self.access_token_expiry_minutes)).timestamp();

        let claims = Claims::new_access_token(user_id, email, exp);

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to generate access token: {e}"))?;

        Ok(token)
    }

    /// Generates a new refresh token for a user
    ///
    /// # Arguments
    /// * `user_id` - The user's UUID
    /// * `email` - The user's email address
    ///
    /// # Returns
    /// A tuple of (signed JWT refresh token as a string, expiration `DateTime`)
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        email: String,
    ) -> anyhow::Result<(String, chrono::DateTime<Utc>)> {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(self.refresh_token_expiry_minutes);
        let exp = expires_at.timestamp();

        let claims = Claims::new_refresh_token(user_id, email, exp);

        let token = encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| anyhow::anyhow!("Failed to generate refresh token: {e}"))?;

        Ok((token, expires_at))
    }

    /// Verifies a JWT token and returns the claims if valid
    ///
    /// # Arguments
    /// * `token` - The JWT token to verify
    ///
    /// # Returns
    /// The decoded Claims if the token is valid
    pub fn verify_token(&self, token: &str) -> anyhow::Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| anyhow::anyhow!("Failed to verify token: {e}"))?;

        let claims = token_data.claims;

        // Check if token is expired
        if claims.is_expired() {
            return Err(anyhow::anyhow!("Token has expired"));
        }

        Ok(claims)
    }

    /// Decodes a JWT token without verifying the signature
    /// ⚠️ WARNING: Only use this when you need to inspect an expired token
    ///
    /// # Arguments
    /// * `token` - The JWT token to decode
    ///
    /// # Returns
    /// The decoded Claims without signature verification
    pub fn decode_token_unverified(&self, token: &str) -> anyhow::Result<Claims> {
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        validation.validate_exp = false;

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(|e| anyhow::anyhow!("Failed to decode token: {e}"))?;

        Ok(token_data.claims)
    }

    /// Extracts user ID from a token without full verification
    /// Useful for logging or metrics
    ///
    /// # Arguments
    /// * `token` - The JWT token
    ///
    /// # Returns
    /// The user UUID if the token can be decoded
    pub fn extract_user_id(&self, token: &str) -> anyhow::Result<Uuid> {
        let claims = self.decode_token_unverified(token)?;
        Ok(claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let service = JwtService::new("test_secret", Some(15), Some(10080));
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        let token = service
            .generate_access_token(user_id, email.clone())
            .unwrap();
        let claims = service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let service = JwtService::new("test_secret", Some(15), Some(10080));
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        let (token, _expires_at) = service
            .generate_refresh_token(user_id, email.clone())
            .unwrap();
        let claims = service.verify_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn test_extract_user_id() {
        let service = JwtService::new("test_secret", Some(15), Some(10080));
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        let token = service.generate_access_token(user_id, email).unwrap();
        let extracted_id = service.extract_user_id(&token).unwrap();

        assert_eq!(extracted_id, user_id);
    }

    #[test]
    fn test_verify_invalid_token() {
        let service = JwtService::new("test_secret", Some(15), Some(10080));
        let result = service.verify_token("invalid_token");

        assert!(result.is_err());
    }

    #[test]
    fn test_different_secrets_fail_verification() {
        let service1 = JwtService::new("secret1", Some(15), Some(10080));
        let service2 = JwtService::new("secret2", Some(15), Some(10080));

        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();

        let token = service1.generate_access_token(user_id, email).unwrap();
        let result = service2.verify_token(&token);

        assert!(result.is_err());
    }
}
