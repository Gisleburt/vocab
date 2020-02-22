struct Translation {
    pub local: String,
    pub foreign: String,
    pub score: u32,
    pub attempts: u32,
}

impl Translation {
    fn new(local: &str, foreign: &str) -> Translation {
        Translation {
            local: local.to_lowercase(),
            foreign: foreign.to_lowercase(),
            score: 0,
            attempts: 0,
        }
    }

    fn get_percent(&self) -> f64 {
        if self.attempts > 0 {
            self.score as f64 / self.attempts as f64
        } else {
            0.0
        }
    }

    fn guess_foreign(&mut self, guess: &str) -> bool {
        self.attempts += 1;
        if self.foreign.to_lowercase() == guess.to_lowercase() {
            self.score += 1;
            true
        } else {
            false
        }
    }

    fn guess_local(&mut self, guess: &str) -> bool {
        self.attempts += 1;
        if self.local.to_lowercase() == guess.to_lowercase() {
            self.score += 1;
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
        assert_eq!(translation.attempts, 1);
        assert_eq!(translation.score, 1);
        assert_eq!(translation.get_percent(), 1.0);

        assert_eq!(translation.guess_local("no"), false);
        assert_eq!(translation.attempts, 2);
        assert_eq!(translation.score, 1);
        assert_eq!(translation.get_percent(), 0.5);
    }

    #[test]
    fn test_guess_foreign() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_foreign("はい"), true);
        assert_eq!(translation.attempts, 1);
        assert_eq!(translation.score, 1);
        assert_eq!(translation.get_percent(), 1.0);

        assert_eq!(translation.guess_foreign("いいえ"), false);
        assert_eq!(translation.attempts, 2);
        assert_eq!(translation.score, 1);
        assert_eq!(translation.get_percent(), 0.5);
    }
}
