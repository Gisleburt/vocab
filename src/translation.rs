use diesel::{Insertable, Queryable};

use crate::schema::translations;

#[derive(Default, Insertable, Queryable)]
pub struct Translation {
    pub local: String,
    pub foreign: String,
    pub guesses_from_local_total: i32,
    pub guesses_from_local_correct: i32,
    pub guesses_from_foreign_total: i32,
    pub guesses_from_foreign_correct: i32,
}

trait GuessLocal {
    fn guess_local(&mut self, guess: &str) -> bool;
}

trait GuessForeign {
    fn guess_foreign(&mut self, guess: &str) -> bool;
}

impl GuessLocal for Translation {
    fn guess_local(&mut self, guess: &str) -> bool {
        self.guesses_from_local_total += 1;
        if self.local.to_lowercase() == guess.to_lowercase() {
            self.guesses_from_local_correct += 1;
            true
        } else {
            false
        }
    }
}

impl GuessForeign for Translation {
    fn guess_foreign(&mut self, guess: &str) -> bool {
        self.guesses_from_foreign_total += 1;
        if self.foreign.to_lowercase() == guess.to_lowercase() {
            self.guesses_from_foreign_correct += 1;
            true
        } else {
            false
        }
    }
}

fn normalised_percent(numerator: i32, denominator: i32) -> f64 {
    if denominator > 0 {
        numerator as f64 / denominator as f64
    } else {
        0.0
    }
}

impl Translation {
    pub fn new(local: &str, foreign: &str) -> Translation {
        Translation {
            local: local.to_lowercase(),
            foreign: foreign.to_lowercase(),
            ..Default::default()
        }
    }

    pub fn get_total_percent(&self) -> f64 {
        normalised_percent(
            self.guesses_from_local_correct + self.guesses_from_foreign_correct,
            self.guesses_from_local_total + self.guesses_from_foreign_total,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{GuessForeign, GuessLocal, Translation};

    #[test]
    fn test_guess_local() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_local("yEs"), true);
        assert_eq!(translation.guesses_from_local_total, 1);
        assert_eq!(translation.guesses_from_local_correct, 1);
        assert_eq!(translation.guesses_from_foreign_total, 0);
        assert_eq!(translation.guesses_from_foreign_correct, 0);
        assert_eq!(translation.get_total_percent(), 1.0);

        assert_eq!(translation.guess_local("no"), false);
        assert_eq!(translation.guesses_from_local_total, 2);
        assert_eq!(translation.guesses_from_local_correct, 1);
        assert_eq!(translation.guesses_from_foreign_total, 0);
        assert_eq!(translation.guesses_from_foreign_correct, 0);
        assert_eq!(translation.get_total_percent(), 0.5);
    }

    #[test]
    fn test_guess_foreign() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_foreign("はい"), true);
        assert_eq!(translation.guesses_from_local_total, 0);
        assert_eq!(translation.guesses_from_local_correct, 0);
        assert_eq!(translation.guesses_from_foreign_total, 1);
        assert_eq!(translation.guesses_from_foreign_correct, 1);
        assert_eq!(translation.get_total_percent(), 1.0);

        assert_eq!(translation.guess_foreign("いいえ"), false);
        assert_eq!(translation.guesses_from_local_total, 0);
        assert_eq!(translation.guesses_from_local_correct, 0);
        assert_eq!(translation.guesses_from_foreign_total, 2);
        assert_eq!(translation.guesses_from_foreign_correct, 1);
        assert_eq!(translation.get_total_percent(), 0.5);
    }
}
