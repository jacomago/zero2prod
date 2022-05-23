-- Add migration script here
ALTER TABLE issue_delivery_queue ADD COLUMN n_retries SMALLINT NULL;
ALTER TABLE issue_delivery_queue ADD COLUMN execute_after timestamptz NULL;