-- Add up migration script here
CREATE TABLE users (
    discordId INTEGER PRIMARY KEY,
    userName TEXT NOT NULL,
    displayName TEXT NOT NULL,
    ban BOOLEAN DEFAULT false,
    createAt DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    updateAt DATE DEFAULT NULL
);