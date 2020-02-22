pub struct Translation {
    pub local: String,
    pub foreign: String,
    pub guesses_correct: u32,
    pub guesses_total: u32,
}

impl Translation {
    pub fn new(local: &str, foreign: &str) -> Translation {
        Translation {
            local: local.to_lowercase(),
            foreign: foreign.to_lowercase(),
            guesses_correct: 0,
            guesses_total: 0,
        }
    }

    pub fn get_percent(&self) -> f64 {
        if self.guesses_total > 0 {
            self.guesses_correct as f64 / self.guesses_total as f64
        } else {
            0.0
        }
    }

    pub fn guess_foreign(&mut self, guess: &str) -> bool {
        self.guesses_total += 1;
        if self.foreign.to_lowercase() == guess.to_lowercase() {
            self.guesses_correct += 1;
            true
        } else {
            false
        }
    }

    pub fn guess_local(&mut self, guess: &str) -> bool {
        self.guesses_total += 1;
        if self.local.to_lowercase() == guess.to_lowercase() {
            self.guesses_correct += 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Translation;

    #[test]
    fn test_guess_local() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_local("yEs"), true);
        assert_eq!(translation.guesses_total, 1);
        assert_eq!(translation.guesses_correct, 1);
        assert_eq!(translation.get_percent(), 1.0);

        assert_eq!(translation.guess_local("no"), false);
        assert_eq!(translation.guesses_total, 2);
        assert_eq!(translation.guesses_correct, 1);
        assert_eq!(translation.get_percent(), 0.5);
    }

    #[test]
    fn test_guess_foreign() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_foreign("はい"), true);
        assert_eq!(translation.guesses_total, 1);
        assert_eq!(translation.guesses_correct, 1);
        assert_eq!(translation.get_percent(), 1.0);

        assert_eq!(translation.guess_foreign("いいえ"), false);
        assert_eq!(translation.guesses_total, 2);
        assert_eq!(translation.guesses_correct, 1);
        assert_eq!(translation.get_percent(), 0.5);
    }
}
