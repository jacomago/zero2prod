-- Add migration script here
-- Wrap migration in transaction to make sure it succeds
-- or fails atomically.
BEGIN;
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Make 'status' mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
