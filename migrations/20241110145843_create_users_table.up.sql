-- Add up migration script here
CREATE TABLE users (
    discord_id INTEGER PRIMARY KEY,
    user_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    ban BOOLEAN DEFAULT false,
    create_at DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    update_at DATE DEFAULT NULL
);