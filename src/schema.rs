table! {
    vocab (local, foreign) {
        local -> Text,
        foreign -> Text,
        guesses_total -> Integer,
        guesses_correct -> Integer,
    }
}
