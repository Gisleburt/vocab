table! {
    translations (local, foreign) {
        local -> Text,
        foreign -> Text,
        guesses_local_total -> Integer,
        guesses_local_correct -> Integer,
        guesses_foreign_total -> Integer,
        guesses_foreign_correct -> Integer,
    }
}
