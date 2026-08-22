CREATE TABLE IF NOT EXISTS "User" (
    "id"                INTEGER,
    "username"          TEXT UNIQUE NOT NULL,
    "email"             TEXT UNIQUE NOT NULL,
    "hashed_password"     TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
