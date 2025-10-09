-- Migration: Create budgets table
-- Created: 2025-01-01

-- Create budgets table
CREATE TABLE IF NOT EXISTS budgets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category TEXT NOT NULL,
    amount BIGINT NOT NULL,
    period TEXT NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_budgets_user_id ON budgets(user_id);
CREATE INDEX IF NOT EXISTS idx_budgets_is_active ON budgets(is_active);
CREATE INDEX IF NOT EXISTS idx_budgets_start_date ON budgets(start_date);
CREATE INDEX IF NOT EXISTS idx_budgets_user_id_active ON budgets(user_id, is_active);
CREATE INDEX IF NOT EXISTS idx_budgets_category ON budgets(category);

-- Add comments
COMMENT ON TABLE budgets IS 'Budget tracking for users';
COMMENT ON COLUMN budgets.period IS 'JSON serialized BudgetPeriod enum (Daily, Weekly, Monthly, Yearly)';
COMMENT ON COLUMN budgets.amount IS 'Budget amount in smallest currency unit (cents)';
COMMENT ON COLUMN budgets.category IS 'Budget category (e.g., "Groceries", "Entertainment")';
COMMENT ON COLUMN budgets.start_date IS 'When the budget period starts';
COMMENT ON COLUMN budgets.end_date IS 'When the budget was deactivated (NULL if still active)';
