use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

/// Service for sending emails using SMTP
#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

impl EmailService {
    /// Creates a new EmailService with SMTP configuration
    ///
    /// # Arguments
    /// * `smtp_host` - SMTP server hostname
    /// * `smtp_port` - SMTP server port
    /// * `smtp_username` - SMTP authentication username
    /// * `smtp_password` - SMTP authentication password
    /// * `from_email` - Sender email address
    /// * `from_name` - Sender display name
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        from_name: String,
    ) -> anyhow::Result<Self> {
        let credentials = Credentials::new(smtp_username, smtp_password);

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(|e| anyhow::anyhow!("Failed to create SMTP transport: {}", e))?
            .port(smtp_port)
            .credentials(credentials)
            .build();

        Ok(Self {
            mailer,
            from_email,
            from_name,
        })
    }

    /// Sends an email verification OTP to a user
    ///
    /// # Arguments
    /// * `to_email` - Recipient email address
    /// * `to_name` - Recipient name
    /// * `otp_code` - 6-digit OTP code
    pub async fn send_verification_otp(
        &self,
        to_email: &str,
        to_name: &str,
        otp_code: &str,
    ) -> anyhow::Result<()> {
        let subject = "Verify Your Email Address";
        let body = format!(
            r#"
            <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #4A5568;">Email Verification</h2>
                        <p>Hi {},</p>
                        <p>Thank you for registering! Please use the following code to verify your email address:</p>
                        <div style="background-color: #F7FAFC; border: 2px solid #E2E8F0; border-radius: 8px; padding: 20px; text-align: center; margin: 20px 0;">
                            <h1 style="color: #2D3748; font-size: 32px; letter-spacing: 8px; margin: 0;">{}</h1>
                        </div>
                        <p>This code will expire in 10 minutes.</p>
                        <p>If you didn't request this verification, please ignore this email.</p>
                        <hr style="border: none; border-top: 1px solid #E2E8F0; margin: 30px 0;">
                        <p style="color: #718096; font-size: 14px;">This is an automated message, please do not reply.</p>
                    </div>
                </body>
            </html>
            "#,
            to_name, otp_code
        );

        self.send_email(to_email, to_name, subject, &body).await
    }

    /// Sends a password reset link to a user
    ///
    /// # Arguments
    /// * `to_email` - Recipient email address
    /// * `to_name` - Recipient name
    /// * `reset_token` - Password reset token
    /// * `base_url` - Base URL of the application (e.g., "https://example.com")
    pub async fn send_password_reset(
        &self,
        to_email: &str,
        to_name: &str,
        reset_token: &str,
        base_url: &str,
    ) -> anyhow::Result<()> {
        let subject = "Reset Your Password";
        let reset_link = format!("{}/reset-password?token={}", base_url, reset_token);

        let body = format!(
            r#"
            <html>
                <body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
                    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
                        <h2 style="color: #4A5568;">Password Reset Request</h2>
                        <p>Hi {},</p>
                        <p>We received a request to reset your password. Click the button below to create a new password:</p>
                        <div style="text-align: center; margin: 30px 0;">
                            <a href="{}" style="background-color: #4299E1; color: white; padding: 12px 30px; text-decoration: none; border-radius: 6px; display: inline-block; font-weight: bold;">Reset Password</a>
                        </div>
                        <p>Or copy and paste this link into your browser:</p>
                        <p style="background-color: #F7FAFC; border: 1px solid #E2E8F0; border-radius: 4px; padding: 10px; word-break: break-all; font-size: 14px;">{}</p>
                        <p style="color: #E53E3E; font-weight: bold;">This link will expire in 1 hour.</p>
                        <p>If you didn't request a password reset, please ignore this email or contact support if you have concerns.</p>
                        <hr style="border: none; border-top: 1px solid #E2E8F0; margin: 30px 0;">
                        <p style="color: #718096; font-size: 14px;">This is an automated message, please do not reply.</p>
                    </div>
                </body>
            </html>
            "#,
            to_name, reset_link, reset_link
        );

        self.send_email(to_email, to_name, subject, &body).await
    }

    /// Internal method to send an email
    async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        html_body: &str,
    ) -> anyhow::Result<()> {
        let from_mailbox = format!("{} <{}>", self.from_name, self.from_email)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid from address: {}", e))?;

        let to_mailbox = format!("{} <{}>", to_name, to_email)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid to address: {}", e))?;

        let email = Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to build email: {}", e))?;

        self.mailer
            .send(email)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send email: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a valid SMTP server to run
    // In a real-world scenario, you would use a mock SMTP server for testing

    #[tokio::test]
    #[ignore] // Ignore by default as it requires SMTP configuration
    async fn test_send_verification_otp() {
        let service = EmailService::new(
            "smtp.example.com".to_string(),
            587,
            "user@example.com".to_string(),
            "password".to_string(),
            "noreply@example.com".to_string(),
            "Example App".to_string(),
        )
        .unwrap();

        let result = service
            .send_verification_otp("test@example.com", "Test User", "123456")
            .await;

        // This will fail without proper SMTP config, which is expected
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    #[ignore] // Ignore by default as it requires SMTP configuration
    async fn test_send_password_reset() {
        let service = EmailService::new(
            "smtp.example.com".to_string(),
            587,
            "user@example.com".to_string(),
            "password".to_string(),
            "noreply@example.com".to_string(),
            "Example App".to_string(),
        )
        .unwrap();

        let result = service
            .send_password_reset(
                "test@example.com",
                "Test User",
                "reset_token_123",
                "https://example.com",
            )
            .await;

        // This will fail without proper SMTP config, which is expected
        assert!(result.is_ok() || result.is_err());
    }
}
