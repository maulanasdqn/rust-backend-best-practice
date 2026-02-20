-- Migration: Create accounts table
-- Created: 2025-01-01

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    account_type TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'USD',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_accounts_is_active ON accounts(is_active);
CREATE INDEX IF NOT EXISTS idx_accounts_user_id_active ON accounts(user_id, is_active);

-- Add comments
COMMENT ON TABLE accounts IS 'Financial accounts belonging to users';
COMMENT ON COLUMN accounts.account_type IS 'JSON serialized AccountType enum (Checking, Savings, Credit, Cash, Investment)';
COMMENT ON COLUMN accounts.balance IS 'Balance in smallest currency unit (cents for USD)';
COMMENT ON COLUMN accounts.currency IS 'ISO 4217 currency code (e.g., USD, EUR, GBP)';
