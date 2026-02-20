-- Migration: Create transactions table
-- Created: 2025-01-01

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    transaction_type TEXT NOT NULL,
    amount BIGINT NOT NULL,
    category TEXT,
    description TEXT,
    transaction_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_transactions_account_id ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_transaction_date ON transactions(transaction_date DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category);
CREATE INDEX IF NOT EXISTS idx_transactions_account_date ON transactions(account_id, transaction_date DESC);

-- Add comments
COMMENT ON TABLE transactions IS 'Financial transactions for accounts';
COMMENT ON COLUMN transactions.transaction_type IS 'JSON serialized TransactionType enum (Income, Expense, Transfer)';
COMMENT ON COLUMN transactions.amount IS 'Transaction amount in smallest currency unit (cents)';
COMMENT ON COLUMN transactions.category IS 'Optional category for organizing transactions (e.g., "Groceries", "Rent")';
COMMENT ON COLUMN transactions.transaction_date IS 'Date when the transaction occurred (may differ from created_at)';
