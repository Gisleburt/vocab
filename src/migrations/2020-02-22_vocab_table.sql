CREATE TABLE vocab
(
    "local"           VARCHAR          NOT NULL,
    "foreign"         VARCHAR          NOT NULL,
    "guesses_total"   UNSIGNED INTEGER NOT NULL,
    "guesses_correct" UNSIGNED INTEGER NOT NULL,
    primary key ("local", "foreign")
);
