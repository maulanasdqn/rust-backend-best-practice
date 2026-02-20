-- Migration: Create email_verifications table
-- Created: 2025-01-01

-- Create email_verifications table for email OTP verification
CREATE TABLE IF NOT EXISTS email_verifications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    otp_code TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_email_verifications_user_id ON email_verifications(user_id);
CREATE INDEX IF NOT EXISTS idx_email_verifications_expires_at ON email_verifications(expires_at);

-- Add comments
COMMENT ON TABLE email_verifications IS 'Stores email verification OTP codes';
COMMENT ON COLUMN email_verifications.otp_code IS '6-digit OTP code for email verification';
COMMENT ON COLUMN email_verifications.expires_at IS 'Expiration timestamp for the OTP code (typically 10 minutes)';
