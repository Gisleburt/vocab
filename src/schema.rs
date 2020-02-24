table! {
    translations (local, foreign) {
        local -> Text,
        foreign -> Text,
        guesses_from_local_total -> Integer,
        guesses_from_local_correct -> Integer,
        guesses_from_foreign_total -> Integer,
        guesses_from_foreign_correct -> Integer,
    }
}
