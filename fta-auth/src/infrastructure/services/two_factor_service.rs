use base32::Alphabet;
use qrcode::{render::svg, QrCode};
use rand::Rng;
use totp_rs::{Algorithm, TOTP};

#[derive(Clone, Debug)]
pub struct TwoFactorService {
    issuer: String,
}

impl TwoFactorService {
    pub const fn new(issuer: String) -> Self {
        Self { issuer }
    }

    pub fn generate_secret(&self) -> String {
        let mut rng = rand::rng();
        let secret_bytes: Vec<u8> = (0..20).map(|_| rng.random::<u8>()).collect();

        base32::encode(Alphabet::Rfc4648 { padding: false }, &secret_bytes)
    }

    pub fn generate_qr_code(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        let otpauth_url = self.get_provisioning_uri(email, secret)?;

        let qr_code = QrCode::new(otpauth_url.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to generate QR code: {e}"))?;

        let svg = qr_code
            .render::<svg::Color>()
            .min_dimensions(200, 200)
            .max_dimensions(400, 400)
            .build();

        Ok(svg)
    }

    pub fn verify_code(&self, email: &str, secret: &str, code: &str) -> anyhow::Result<bool> {
        let totp = Self::create_totp(email, secret)?;

        Ok(totp.check_current(code).unwrap_or(false))
    }

    pub fn generate_current_code(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        let totp = Self::create_totp(email, secret)?;
        totp.generate_current()
            .map_err(|e| anyhow::anyhow!("Failed to generate code: {e}"))
    }

    pub fn get_provisioning_uri(&self, email: &str, secret: &str) -> anyhow::Result<String> {
        let uri = format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30",
            urlencoding::encode(&self.issuer),
            urlencoding::encode(email),
            secret,
            urlencoding::encode(&self.issuer)
        );
        Ok(uri)
    }

    fn create_totp(_email: &str, secret: &str) -> anyhow::Result<TOTP> {
        let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: false }, secret)
            .ok_or_else(|| anyhow::anyhow!("Failed to decode secret"))?;

        TOTP::new(Algorithm::SHA1, 6, 1, 30, secret_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to create TOTP: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        assert!(!secret.is_empty());
        assert!(secret.len() >= 16);
    }

    #[test]
    fn test_generate_qr_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let qr_code = service
            .generate_qr_code("test@example.com", &secret)
            .unwrap();

        assert!(qr_code.contains("<svg"));
        assert!(qr_code.contains("</svg>"));
    }

    #[test]
    fn test_verify_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();
        let email = "test@example.com";

        let code = service.generate_current_code(email, &secret).unwrap();

        let is_valid = service.verify_code(email, &secret, &code).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_verify_invalid_code() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let is_valid = service
            .verify_code("test@example.com", &secret, "000000")
            .unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_get_provisioning_uri() {
        let service = TwoFactorService::new("Test App".to_string());
        let secret = service.generate_secret();

        let uri = service
            .get_provisioning_uri("test@example.com", &secret)
            .unwrap();

        assert!(uri.starts_with("otpauth://totp/"));
        assert!(uri.contains("test@example.com"));
    }
}
