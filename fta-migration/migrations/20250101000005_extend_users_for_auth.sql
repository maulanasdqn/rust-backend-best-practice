-- Migration: Extend users table for authentication features
-- Created: 2025-01-01

-- Add authentication-related columns to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS two_factor_enabled BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS two_factor_secret TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_provider TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS oauth_provider_id TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ;

-- Add index for OAuth lookups
CREATE INDEX IF NOT EXISTS idx_users_oauth_provider ON users(oauth_provider, oauth_provider_id);

-- Add comments
COMMENT ON COLUMN users.email_verified IS 'Whether the user has verified their email address';
COMMENT ON COLUMN users.two_factor_enabled IS 'Whether 2FA (TOTP) is enabled for this user';
COMMENT ON COLUMN users.two_factor_secret IS 'TOTP secret for 2FA (base32 encoded)';
COMMENT ON COLUMN users.oauth_provider IS 'OAuth provider name (e.g., "google")';
COMMENT ON COLUMN users.oauth_provider_id IS 'User ID from OAuth provider';
COMMENT ON COLUMN users.last_login_at IS 'Timestamp of last successful login';
