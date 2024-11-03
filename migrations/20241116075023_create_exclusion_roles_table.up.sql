-- Add up migration script here
CREATE TABLE exclusion_roles (
    id INTEGER PRIMARY KEY,
    roleID INTEGER UNIQUE,
    memo TEXT DEFAULT NULL,
    createAt DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    updateAt DATE DEFAULT NULL
);

INSERT INTO exclusion_roles (roleID, memo)
VALUES (1295596580790866002, '管理者');

INSERT INTO exclusion_roles (roleID, memo)
VALUES (1295615265316405319, 'サキュバス');