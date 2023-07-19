-- Add migration script here
ALTER TABLE todos
ADD COLUMN IF NOT EXISTS description TEXT;