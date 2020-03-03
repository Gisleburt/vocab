use diesel::{Insertable, Queryable};

use crate::schema::translations;
use crate::VocabStoreError;

#[derive(Debug, Default, Insertable, Queryable, PartialEq)]
pub struct Translation {
    pub local: String,
    pub foreign: String,
    pub guesses_local_total: i32,
    pub guesses_local_correct: i32,
    pub guesses_foreign_total: i32,
    pub guesses_foreign_correct: i32,
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
            self.guesses_local_correct + self.guesses_foreign_correct,
            self.guesses_local_total + self.guesses_foreign_total,
        )
    }

    pub fn guess_local(&mut self, guess: &str) -> bool {
        self.guesses_local_total += 1;
        if self.local.to_lowercase() == guess.to_lowercase() {
            self.guesses_local_correct += 1;
            true
        } else {
            false
        }
    }

    pub fn guess_foreign(&mut self, guess: &str) -> bool {
        self.guesses_foreign_total += 1;
        if self.foreign.to_lowercase() == guess.to_lowercase() {
            self.guesses_foreign_correct += 1;
            true
        } else {
            false
        }
    }

    pub fn reconcile(self, other: Translation) -> Result<Translation, VocabStoreError> {
        // Don't reconcile different translations
        if self.local != other.local || self.foreign != other.foreign {
            return Err(VocabStoreError::ReconciliationError);
        }

        // Take whichever side has most guesses
        let more_local = self.guesses_local_total > other.guesses_local_total;
        let more_foreign = self.guesses_foreign_total > other.guesses_foreign_total;

        let (guesses_local_total, guesses_local_correct) = if more_local {
            (self.guesses_local_total, self.guesses_local_correct)
        } else {
            (other.guesses_local_total, other.guesses_local_correct)
        };
        let (guesses_foreign_total, guesses_foreign_correct) = if more_foreign {
            (self.guesses_foreign_total, self.guesses_foreign_correct)
        } else {
            (other.guesses_foreign_total, other.guesses_foreign_correct)
        };

        Ok(Translation {
            local: self.local,
            foreign: self.foreign,
            guesses_local_total,
            guesses_local_correct,
            guesses_foreign_total,
            guesses_foreign_correct,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Translation;
    use crate::VocabStoreError;

    #[test]
    fn test_guess_local() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_local("yEs"), true);
        assert_eq!(translation.guesses_local_total, 1);
        assert_eq!(translation.guesses_local_correct, 1);
        assert_eq!(translation.guesses_foreign_total, 0);
        assert_eq!(translation.guesses_foreign_correct, 0);
        assert_eq!(translation.get_total_percent(), 1.0);

        assert_eq!(translation.guess_local("no"), false);
        assert_eq!(translation.guesses_local_total, 2);
        assert_eq!(translation.guesses_local_correct, 1);
        assert_eq!(translation.guesses_foreign_total, 0);
        assert_eq!(translation.guesses_foreign_correct, 0);
        assert_eq!(translation.get_total_percent(), 0.5);
    }

    #[test]
    fn test_guess_foreign() {
        let mut translation = Translation::new("yes", "はい");
        assert_eq!(translation.guess_foreign("はい"), true);
        assert_eq!(translation.guesses_local_total, 0);
        assert_eq!(translation.guesses_local_correct, 0);
        assert_eq!(translation.guesses_foreign_total, 1);
        assert_eq!(translation.guesses_foreign_correct, 1);
        assert_eq!(translation.get_total_percent(), 1.0);

        assert_eq!(translation.guess_foreign("いいえ"), false);
        assert_eq!(translation.guesses_local_total, 0);
        assert_eq!(translation.guesses_local_correct, 0);
        assert_eq!(translation.guesses_foreign_total, 2);
        assert_eq!(translation.guesses_foreign_correct, 1);
        assert_eq!(translation.get_total_percent(), 0.5);
    }

    #[test]
    fn test_reconcile() {
        let mut old_translation = Translation::new("yes", "はい");
        old_translation.guesses_local_correct = 4;
        old_translation.guesses_local_total = 5;
        old_translation.guesses_foreign_correct = 3;
        old_translation.guesses_foreign_total = 5;
        let mut new_translation = Translation::new("yes", "はい");
        new_translation.guesses_local_correct = 4;
        new_translation.guesses_local_total = 4;
        new_translation.guesses_foreign_correct = 6;
        new_translation.guesses_foreign_total = 6;
        let reconciled_translation = old_translation.reconcile(new_translation).unwrap();

        assert_eq!(reconciled_translation.local, "yes");
        assert_eq!(reconciled_translation.foreign, "はい");
        assert_eq!(reconciled_translation.guesses_local_correct, 4);
        assert_eq!(reconciled_translation.guesses_local_total, 5);
        assert_eq!(reconciled_translation.guesses_foreign_correct, 6);
        assert_eq!(reconciled_translation.guesses_foreign_total, 6);

        let mut old_translation = Translation::new("no", "いいえ");
        let mut new_translation = Translation::new("japan", "日本");
        match old_translation.reconcile(new_translation) {
            Err(VocabStoreError::ReconciliationError) => {}
            _ => assert!(false, "VocabStore did not return ReconciliationError error"),
        }
    }
}
