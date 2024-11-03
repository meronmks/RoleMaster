-- Add up migration script here
CREATE TABLE reaction_emoji_settings (
    id INTEGER PRIMARY KEY,
    instanceNum INTEGER UNIQUE,
    isCustomEmoji BOOLEAN NOT NULL,
    unicodeEmoji TEXT DEFAULT NULL,
    isAnimatedCustomEmoji BOOLEAN DEFAULT false,
    customEmojiId INTEGER DEFAULT 0,
    customEmojiName TEXT DEFAULT NULL
);

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (1, '0', '1⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (2, '0', '2⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (3, '0', '3⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (4, '0', '4⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (5, '0', '5⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (6, '0', '6⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (7, '0', '7⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (8, '0', '8⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (9, '0', '9⃣');

INSERT INTO reaction_emoji_settings (instanceNum, isCustomEmoji, unicodeEmoji)
VALUES (10, '0', '🔟');