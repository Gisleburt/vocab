CREATE TABLE translations
(
    "local"                        VARCHAR          NOT NULL,
    "foreign"                      VARCHAR          NOT NULL,
    "guesses_from_local_total"     UNSIGNED INTEGER NOT NULL,
    "guesses_from_local_correct"   UNSIGNED INTEGER NOT NULL,
    "guesses_from_foreign_total"   UNSIGNED INTEGER NOT NULL,
    "guesses_from_foreign_correct" UNSIGNED INTEGER NOT NULL,
    primary key ("local", "foreign"),
    unique (local),
    unique ("foreign")
);
