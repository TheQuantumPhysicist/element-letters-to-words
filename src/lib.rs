use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Solution {
    target_word: String,
    reconstructed_word: Vec<String>,
    used_letters: BTreeSet<usize>,
}

impl Solution {
    pub fn new(target_word: impl Into<String>) -> Self {
        Self {
            target_word: target_word.into(),
            reconstructed_word: Vec::default(),
            used_letters: BTreeSet::default(),
        }
    }

    /// Extend the current solution by adding letters. This can have the effect of forking the current solution to multiple ones.
    /// This is because multiple elements can be simultaneously valid
    pub fn extend(self, elements_possibilities: &[(usize, &String)]) -> Vec<Solution> {
        let constructed_so_far = self.get_constructed_so_far();

        let mut result = vec![];

        // for all available possibilities
        for (possibility_idx, possibility) in elements_possibilities {
            let already_used_el = self.used_letters.contains(possibility_idx);
            let concat_matches = self
                .target_word
                .starts_with(&(constructed_so_far.clone() + possibility));

            if concat_matches && !already_used_el {
                let mut sol = self.clone();

                sol.used_letters.insert(*possibility_idx);
                sol.reconstructed_word.push((*possibility).clone());
                result.push(sol);
            }
        }

        // Check the case where no need to add letters and we already have a match
        if self.target_word.starts_with(&constructed_so_far) {
            result.push(self.clone())
        }

        result
    }

    fn get_constructed_so_far(&self) -> String {
        self.reconstructed_word
            .iter()
            .map(|s| s.chars())
            .flatten()
            .collect()
    }

    /// Return Some(Self) if the solution is complete, else None
    pub fn finalize(self) -> Option<Self> {
        if self.get_constructed_so_far() == self.target_word {
            Some(self)
        } else {
            None
        }
    }
}

/// Given a set of word segments in `elements`, construct the given `word` using these segments. Reusing segments is not allowed.
/// ```
/// use element_letters_to_word::spell_it;
///
/// let elements = [
///   "ac", "at", "c", "ca", "f", "h", "k", "o", "p", "po", "s", "sc", "se", "tc", "te",
///   "y", "yt",
/// ].into_iter()
///  .map(|el| el.to_string())
///  .collect::<Vec<_>>();
///
/// assert_eq!(spell_it("cat", &elements).unwrap().join(""), "cat");
/// assert_eq!(spell_it("spooky", &elements).unwrap().join(""), "spooky");
/// ```
///
pub fn spell_it(word: &str, elements: &[String]) -> Option<Vec<String>> {
    assert!(
        word.is_ascii() && word.chars().all(|ch| ch.is_alphabetic()),
        "The provided word must be ASCII alphabet"
    );
    assert!(
        elements.iter().all(|s| s.is_ascii()
            && s.chars().all(|ch| ch.is_alphabetic())
            && s.chars().all(|ch| ch.is_lowercase())),
        "All the elements are expected to be ASCII letters and lowercase"
    );

    let mut all_solutions = vec![Solution::new(word.to_lowercase())];

    let word_chars = word.to_lowercase().chars().collect::<Vec<_>>();

    for to_skip in 0..word_chars.len() {
        // Regardless of the length of the element, we just check that a its letters matches the beginning of the current suffix
        let word_suffix = word_chars.iter().copied().skip(to_skip).collect::<String>();

        // This means we're done; avoiding the special case of skipping the whole word
        if word_suffix.is_empty() {
            break;
        }

        let fitting_elements = elements
            .iter()
            .enumerate()
            .filter(|(_idx, el)| word_suffix.starts_with(*el))
            .collect::<Vec<_>>();

        all_solutions = all_solutions
            .into_iter()
            .map(|sol| sol.extend(&fitting_elements))
            .flatten()
            .collect();
    }

    // There are potentially many solutions, we just take the first one
    all_solutions
        .into_iter()
        .map(|sol| sol.finalize())
        .flatten()
        .next()
        .map(|sol| sol.reconstructed_word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail() {
        let words = ["possess", "posses", "attack", "koko", "skate", "scoop"];

        for word in words {
            let elements = [
                "ac", "at", "c", "ca", "f", "h", "k", "o", "p", "po", "s", "sc", "se", "tc", "te",
                "y", "yt",
            ]
            .into_iter()
            .map(|el| el.to_string())
            .collect::<Vec<_>>();

            let sol = spell_it(word, &elements);
            assert_eq!(sol, None);
        }
    }

    #[test]
    fn many_words() {
        let words = [
            "spooky", "cat", "chat", "pat", "fat", "at", "pack", "hat", "capo", "pose", "coach",
            "case", "cos", "cop", "hack", "tesse", "hose", "caco", "posse",
        ];

        for word in words {
            let elements = [
                "ac", "at", "c", "ca", "f", "h", "k", "o", "p", "po", "s", "sc", "se", "tc", "te",
                "y", "yt",
            ]
            .into_iter()
            .map(|el| el.to_string())
            .collect::<Vec<_>>();

            let sol = spell_it(word, &elements);
            println!("{:?}", sol.as_ref().unwrap());
            assert_eq!(sol.unwrap().join(""), word);
        }
    }
}
