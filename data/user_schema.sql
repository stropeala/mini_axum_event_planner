CREATE TABLE IF NOT EXISTS "User" (
    "id"                INTEGER,
    "username"          TEXT UNIQUE NOT NULL,
    "email"             TEXT UNIQUE NOT NULL,
    "password_hash"     TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
