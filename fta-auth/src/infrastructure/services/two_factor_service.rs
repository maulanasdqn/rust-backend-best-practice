use base32::Alphabet;
use qrcode::{QrCode, render::svg};
use rand::Rng;
use totp_rs::{Algorithm, TOTP};

/// Service for Two-Factor Authentication using TOTP
#[derive(Clone)]
pub struct TwoFactorService {
    issuer: String,
}

impl TwoFactorService {
    /// Creates a new TwoFactorService
    ///
    /// # Arguments
    /// * `issuer` - The name of your application (e.g., "Financial Tracker")
    pub fn new(issuer: String) -> Self {
        Self { issuer }
    }

    /// Generates a new TOTP secret for a user
    ///
    /// # Returns
    /// The base32-encoded secret string that should be stored in the database
    pub fn generate_secret(&self) -> String {
        // Generate random 20-byte secret
        let mut rng = rand::rng();
        let secret_bytes: Vec<u8> = (0..20).map(|_| rng.random::<u8>()).collect();

        // Encode as base32
        base32::encode(Alphabet::Rfc4648 { padding: false }, &secret_bytes)
    }

    /// Generates a QR code SVG for TOTP setup
    ///
    /// # Arguments
    /// * `email` - The user's email address
    /// * `secret` - The base32-encoded TOTP secret
    ///
    /// # Returns
    /// An SVG string representing the QR code
    pub fn generate_qr_code(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        let otpauth_url = self.get_provisioning_uri(email, secret)?;

        let qr_code = QrCode::new(otpauth_url.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to generate QR code: {}", e))?;

        let svg = qr_code
            .render::<svg::Color>()
            .min_dimensions(200, 200)
            .max_dimensions(400, 400)
            .build();

        Ok(svg)
    }

    /// Verifies a TOTP code against a secret
    ///
    /// # Arguments
    /// * `email` - The user's email address
    /// * `secret` - The base32-encoded TOTP secret
    /// * `code` - The 6-digit TOTP code to verify
    ///
    /// # Returns
    /// true if the code is valid, false otherwise
    pub fn verify_code(&self, email: &str, secret: &str, code: &str) -> anyhow::Result<bool> {
        let totp = self.create_totp(email, secret)?;

        Ok(totp.check_current(code).unwrap_or(false))
    }

    /// Generates the current TOTP code for testing purposes
    /// ⚠️ This should only be used in development/testing
    ///
    /// # Arguments
    /// * `email` - The user's email address
    /// * `secret` - The base32-encoded TOTP secret
    ///
    /// # Returns
    /// The current 6-digit TOTP code
    pub fn generate_current_code(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        let totp = self.create_totp(email, secret)?;
        Ok(totp.generate_current().map_err(|e| anyhow::anyhow!("Failed to generate code: {}", e))?)
    }

    /// Gets the provisioning URI for manual entry
    ///
    /// # Arguments
    /// * `email` - The user's email address
    /// * `secret` - The base32-encoded TOTP secret
    ///
    /// # Returns
    /// The otpauth:// URI string
    pub fn get_provisioning_uri(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        // Build otpauth URL manually
        let uri = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
            urlencoding::encode(&self.issuer),
            urlencoding::encode(email),
            secret,
            urlencoding::encode(&self.issuer)
        );
        Ok(uri)
    }

    /// Creates a TOTP instance from email and secret
    fn create_totp(&self, email: &str, secret: &str) -> anyhow::Result<TOTP> {
        // Decode the base32 secret
        let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: false }, secret)
            .ok_or_else(|| anyhow::anyhow!("Failed to decode secret"))?;

        TOTP::new(
            Algorithm::SHA1,
            6,          // 6 digits
            1,          // 1 step skew
            30,         // 30 second time step
            secret_bytes,
        )
        .map_err(|e| anyhow::anyhow!("Failed to create TOTP: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        // Secret should be a non-empty base32 string
        assert!(!secret.is_empty());
        assert!(secret.len() >= 16);
    }

    #[test]
    fn test_generate_qr_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let qr_code = service.generate_qr_code("test@example.com", &secret).unwrap();

        // QR code should be an SVG
        assert!(qr_code.contains("<svg"));
        assert!(qr_code.contains("</svg>"));
    }

    #[test]
    fn test_verify_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();
        let email = "test@example.com";

        // Generate the current code
        let code = service.generate_current_code(email, &secret).unwrap();

        // Verify it
        let is_valid = service.verify_code(email, &secret, &code).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_invalid_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let is_valid = service.verify_code("test@example.com", &secret, "000000").unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_get_provisioning_uri() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let uri = service.get_provisioning_uri("test@example.com", &secret).unwrap();

        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("test@example.com"));
    }
}
