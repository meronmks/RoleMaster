-- Add up migration script here
CREATE TABLE user_miss_count (
    discord_id INTEGER PRIMARY KEY,
    count INTEGER NOT NULL,
    create_at DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    update_at DATE DEFAULT NULL
);