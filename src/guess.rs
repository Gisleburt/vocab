use std::ops::Deref;

use crate::Translation;

pub enum Guess {
    Local(Translation),
    Foreign(Translation),
}

impl Guess {
    pub fn render(&self) -> &str {
        match self {
            Guess::Local(translation) => translation.foreign.as_str(),
            Guess::Foreign(translation) => translation.local.as_str(),
        }
    }

    pub fn guess(&mut self, guess: &str) -> bool {
        match self {
            Guess::Local(ref mut translation) => translation.guess_local(guess),
            Guess::Foreign(ref mut translation) => translation.guess_foreign(guess),
        }
    }
}

impl Deref for Guess {
    type Target = Translation;

    fn deref(&self) -> &Self::Target {
        match self {
            Guess::Local(translation) => translation,
            Guess::Foreign(translation) => translation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Guess, Translation};

    #[test]
    fn test_render_local() {
        let translation = Translation::new("yes", "はい");
        let guess = Guess::Local(translation);
        assert_eq!(guess.render(), "はい");
    }

    #[test]
    fn test_render_foreign() {
        let translation = Translation::new("yes", "はい");
        let guess = Guess::Foreign(translation);
        assert_eq!(guess.render(), "yes");
    }

    #[test]
    fn test_guess_local() {
        let translation = Translation::new("yes", "はい");
        let mut guess = Guess::Local(translation);
        assert!(guess.guess("yes"));
        assert_eq!(guess.guesses_local_total, 1);
        assert_eq!(guess.guesses_foreign_total, 0);
    }

    #[test]
    fn test_guess_foreign() {
        let translation = Translation::new("yes", "はい");
        let mut guess = Guess::Foreign(translation);
        assert!(guess.guess("はい"));
        assert_eq!(guess.guesses_foreign_total, 1);
        assert_eq!(guess.guesses_local_total, 0);
    }
}
