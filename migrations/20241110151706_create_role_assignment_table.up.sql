-- Add up migration script here
CREATE TABLE role_assignment (
    id INTEGER PRIMARY KEY,
    instance_num INTEGER UNIQUE,
    role_id INTEGER NOT NULL
);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (0, 1295618567798132778);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (1, 1295617660561915906);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (2, 1295618265992794144);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (3, 1295618307587833938);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (4, 1295618370791800883);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (5, 1295618401829654628);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (6, 1295618429805395978);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (7, 1295618453935493142);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (8, 1295618509161893921);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (9, 1295618537976627210);

INSERT INTO role_assignment (instance_num, role_id)
VALUES (10, 1295618685511405658);