-- Add up migration script here
CREATE TABLE exclusion_roles (
    id INTEGER PRIMARY KEY,
    role_id INTEGER UNIQUE,
    memo TEXT DEFAULT NULL,
    create_at DATE DEFAULT CURRENT_TIMESTAMP, -- SQLiteではUTC
    update_at DATE DEFAULT NULL
);

INSERT INTO exclusion_roles (role_id, memo)
VALUES (1295596580790866002, '管理者');

INSERT INTO exclusion_roles (role_id, memo)
VALUES (1295615265316405319, 'サキュバス');