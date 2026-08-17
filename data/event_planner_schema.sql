CREATE TABLE IF NOT EXISTS "EventPlanner" (
    "id"                INTEGER,
    "name"              TEXT NOT NULL,
    "date"              TEXT NOT NULL,
    "time"              TEXT NOT NULL,
    "description"       TEXT NOT NULL,
    PRIMARY KEY("id" AUTOINCREMENT)
) STRICT;
