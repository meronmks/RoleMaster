-- Add up migration script here
CREATE TABLE reaction_emoji_settings (
    id INTEGER PRIMARY KEY,
    instance_num INTEGER UNIQUE,
    is_custom_emoji BOOLEAN NOT NULL,
    unicode_emoji TEXT DEFAULT NULL,
    is_animated_custom_emoji BOOLEAN DEFAULT false,
    custom_emoji_id INTEGER DEFAULT 0,
    custom_emoji_name TEXT DEFAULT NULL
);

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (1, '0', '1⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (2, '0', '2⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (3, '0', '3⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (4, '0', '4⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (5, '0', '5⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (6, '0', '6⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (7, '0', '7⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (8, '0', '8⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (9, '0', '9⃣');

INSERT INTO reaction_emoji_settings (instance_num, is_custom_emoji, unicode_emoji)
VALUES (10, '0', '🔟');