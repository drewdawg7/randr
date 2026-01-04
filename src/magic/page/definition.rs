use crate::magic::spell::{compute_spell, ComputedSpell};
use crate::magic::word::WordId;

// ─────────────────────────────────────────────────────────────────────────────
// Page (spell slot that holds words)
// ─────────────────────────────────────────────────────────────────────────────

/// A page in a tome that can hold up to 5 words
#[derive(Debug, Clone)]
pub struct Page {
    /// The words inscribed on this page
    words: Vec<WordId>,

    /// The computed spell effect (cached after inscription)
    computed_spell: Option<ComputedSpell>,
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}

impl Page {
    /// Maximum number of words per page
    pub const MAX_WORDS: usize = 5;

    /// Create a new empty page
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            computed_spell: None,
        }
    }

    /// Get the words on this page
    pub fn words(&self) -> &[WordId] {
        &self.words
    }

    /// Get the computed spell (if any)
    pub fn spell(&self) -> Option<&ComputedSpell> {
        self.computed_spell.as_ref()
    }

    /// Check if the page is empty
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// Check if the page has a castable spell
    pub fn has_castable_spell(&self) -> bool {
        self.computed_spell
            .as_ref()
            .map(|s| s.is_castable())
            .unwrap_or(false)
    }

    /// Check if the page has a passive effect
    pub fn has_passive(&self) -> bool {
        self.computed_spell
            .as_ref()
            .map(|s| s.is_passive())
            .unwrap_or(false)
    }

    /// Inscribe words onto the page, computing the resulting spell
    pub fn inscribe(&mut self, words: Vec<WordId>) -> InscriptionResult {
        if words.is_empty() {
            return InscriptionResult::Empty;
        }

        if words.len() > Self::MAX_WORDS {
            return InscriptionResult::TooManyWords {
                provided: words.len(),
                max: Self::MAX_WORDS,
            };
        }

        // Store the words
        self.words = words;

        // Compute the spell
        let spell = compute_spell(&self.words);
        let result = match &spell {
            ComputedSpell::Active { name, .. } => InscriptionResult::Success {
                spell_name: name.clone(),
                is_backfire: false,
            },
            ComputedSpell::Passive { name, .. } => InscriptionResult::Success {
                spell_name: name.clone(),
                is_backfire: false,
            },
            ComputedSpell::Hybrid { name, .. } => InscriptionResult::Success {
                spell_name: name.clone(),
                is_backfire: false,
            },
            ComputedSpell::Backfire { reason, .. } => InscriptionResult::Backfire {
                reason: reason.clone(),
            },
            ComputedSpell::Fizzle { reason } => InscriptionResult::Fizzle {
                reason: reason.clone(),
            },
        };

        self.computed_spell = Some(spell);
        result
    }

    /// Clear the page
    pub fn clear(&mut self) {
        self.words.clear();
        self.computed_spell = None;
    }

    /// Try to parse word strings and inscribe them
    pub fn inscribe_from_strings(
        &mut self,
        word_strings: &[&str],
    ) -> Result<InscriptionResult, Vec<String>> {
        let mut words = Vec::new();
        let mut unknown = Vec::new();

        for s in word_strings {
            if let Some(word_id) = WordId::from_str(s) {
                words.push(word_id);
            } else {
                unknown.push(s.to_string());
            }
        }

        if !unknown.is_empty() {
            return Err(unknown);
        }

        Ok(self.inscribe(words))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Inscription Result
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum InscriptionResult {
    /// Successfully inscribed a spell
    Success {
        spell_name: String,
        is_backfire: bool,
    },

    /// The combination caused a backfire
    Backfire { reason: String },

    /// The combination fizzled (no effect)
    Fizzle { reason: String },

    /// No words provided
    Empty,

    /// Too many words
    TooManyWords { provided: usize, max: usize },
}

impl InscriptionResult {
    pub fn is_success(&self) -> bool {
        matches!(self, InscriptionResult::Success { .. })
    }
}
