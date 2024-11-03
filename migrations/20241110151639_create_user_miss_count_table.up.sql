-- Add up migration script here
CREATE TABLE user_miss_count (
    discordId INTEGER PRIMARY KEY,
    count INTEGER NOT NULL,
    createAt DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    updateAt DATE DEFAULT NULL
);