use rand::Rng;

/// Service for generating One-Time Passwords (OTP)
#[derive(Clone, Default, Debug)]
pub struct OtpService;

impl OtpService {
    /// Creates a new `OtpService`
    pub const fn new() -> Self {
        Self
    }

    /// Generates a random 6-digit OTP code
    ///
    /// # Returns
    /// A string containing a 6-digit numeric code
    pub fn generate_code(&self) -> String {
        let mut rng = rand::rng();
        let code = rng.random_range(100_000..=999_999);
        format!("{code:06}")
    }

    /// Validates an OTP code format
    ///
    /// # Arguments
    /// * `code` - The OTP code to validate
    ///
    /// # Returns
    /// true if the code is exactly 6 digits, false otherwise
    pub fn is_valid_format(&self, code: &str) -> bool {
        code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
    }

    /// Compares two OTP codes securely
    /// Uses constant-time comparison to prevent timing attacks
    ///
    /// # Arguments
    /// * `provided` - The code provided by the user
    /// * `expected` - The code expected/stored
    ///
    /// # Returns
    /// true if the codes match, false otherwise
    pub fn verify_code(&self, provided: &str, expected: &str) -> bool {
        if provided.len() != expected.len() {
            return false;
        }

        // Constant-time comparison
        provided
            .bytes()
            .zip(expected.bytes())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code() {
        let service = OtpService::new();
        let code = service.generate_code();

        // Should be exactly 6 digits
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));

        // Should be within valid range
        let code_num: u32 = code.parse().unwrap();
        assert!((100_000..=999_999).contains(&code_num));
    }

    #[test]
    fn test_generate_multiple_codes_are_different() {
        let service = OtpService::new();
        let code1 = service.generate_code();
        let code2 = service.generate_code();

        // While technically they could be the same, probability is very low
        // This test might occasionally fail but helps verify randomness
        assert_ne!(code1, code2);
    }

    #[test]
    fn test_is_valid_format() {
        let service = OtpService::new();

        assert!(service.is_valid_format("123456"));
        assert!(service.is_valid_format("000000"));
        assert!(service.is_valid_format("999999"));

        assert!(!service.is_valid_format("12345")); // Too short
        assert!(!service.is_valid_format("1234567")); // Too long
        assert!(!service.is_valid_format("12345a")); // Contains letter
        assert!(!service.is_valid_format("12 456")); // Contains space
        assert!(!service.is_valid_format("")); // Empty
    }

    #[test]
    fn test_verify_code() {
        let service = OtpService::new();

        assert!(service.verify_code("123456", "123456"));
        assert!(service.verify_code("000000", "000000"));

        assert!(!service.verify_code("123456", "123457"));
        assert!(!service.verify_code("123456", "12345"));
        assert!(!service.verify_code("123456", ""));
    }

    #[test]
    fn test_verify_code_timing_safe() {
        let service = OtpService::new();

        // All these comparisons should take roughly the same time
        // This is a basic test; a proper timing attack test would be more complex
        let tests = vec![
            ("123456", "123456"),
            ("123456", "654321"),
            ("123456", "000000"),
            ("123456", "999999"),
        ];

        for (provided, expected) in tests {
            let _ = service.verify_code(provided, expected);
        }
    }
}
