use crate::magic::effect::PassiveEffect;
use crate::magic::page::Page;
use crate::magic::spell::ComputedSpell;

// ─────────────────────────────────────────────────────────────────────────────
// Tome (holds pages of spells)
// ─────────────────────────────────────────────────────────────────────────────

/// A tome that holds pages of spells
#[derive(Debug, Clone)]
pub struct Tome {
    /// The pages in this tome
    pages: Vec<Option<Page>>,

    /// Which page is currently active for casting
    active_page_index: usize,

    /// Maximum number of pages this tome can hold
    capacity: usize,
}

impl Tome {
    /// Create a new tome with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            pages: vec![None; capacity],
            active_page_index: 0,
            capacity,
        }
    }

    /// Create a standard 3-page tome (for testing)
    pub fn standard() -> Self {
        Self::new(3)
    }

    /// Get the tome's capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get a reference to a page by index
    pub fn page(&self, index: usize) -> Option<&Page> {
        self.pages.get(index).and_then(|p| p.as_ref())
    }

    /// Get a mutable reference to a page by index
    pub fn page_mut(&mut self, index: usize) -> Option<&mut Page> {
        self.pages.get_mut(index).and_then(|p| p.as_mut())
    }

    /// Insert a page at the given index, returning the previous page if any
    pub fn set_page(&mut self, index: usize, page: Page) -> Result<Option<Page>, TomeError> {
        if index >= self.capacity {
            return Err(TomeError::IndexOutOfBounds {
                index,
                capacity: self.capacity,
            });
        }
        let old = self.pages[index].take();
        self.pages[index] = Some(page);
        Ok(old)
    }

    /// Remove a page at the given index
    pub fn remove_page(&mut self, index: usize) -> Option<Page> {
        if index >= self.capacity {
            return None;
        }
        self.pages[index].take()
    }

    /// Get the currently active page
    pub fn active_page(&self) -> Option<&Page> {
        self.page(self.active_page_index)
    }

    /// Get the active page index
    pub fn active_page_index(&self) -> usize {
        self.active_page_index
    }

    /// Set the active page index, returning the previous active page index
    pub fn set_active_page(&mut self, index: usize) -> Result<usize, TomeError> {
        if index >= self.capacity {
            return Err(TomeError::IndexOutOfBounds {
                index,
                capacity: self.capacity,
            });
        }
        let old = self.active_page_index;
        self.active_page_index = index;
        Ok(old)
    }

    /// Cycle to the next page
    pub fn next_page(&mut self) {
        self.active_page_index = (self.active_page_index + 1) % self.capacity;
    }

    /// Cycle to the previous page
    pub fn prev_page(&mut self) {
        if self.active_page_index == 0 {
            self.active_page_index = self.capacity - 1;
        } else {
            self.active_page_index -= 1;
        }
    }

    /// Get the spell from the active page
    pub fn active_spell(&self) -> Option<&ComputedSpell> {
        self.active_page().and_then(|p| p.spell())
    }

    /// Get all passive effects from all pages
    pub fn passive_effects(&self) -> Vec<&PassiveEffect> {
        self.pages
            .iter()
            .filter_map(|p| p.as_ref())
            .filter_map(|page| {
                page.spell().and_then(|spell| match spell {
                    ComputedSpell::Passive { effect, .. } => Some(effect),
                    ComputedSpell::Hybrid { passive, .. } => Some(passive),
                    _ => None,
                })
            })
            .collect()
    }

    /// Get all passive effects with their spell names
    pub fn passive_effects_with_names(&self) -> Vec<(&str, &PassiveEffect)> {
        self.pages
            .iter()
            .filter_map(|p| p.as_ref())
            .filter_map(|page| {
                page.spell().and_then(|spell| match spell {
                    ComputedSpell::Passive { name, effect, .. } => Some((name.as_str(), effect)),
                    ComputedSpell::Hybrid { name, passive, .. } => Some((name.as_str(), passive)),
                    _ => None,
                })
            })
            .collect()
    }

    /// Count how many pages are inscribed
    pub fn inscribed_count(&self) -> usize {
        self.pages.iter().filter(|p| p.is_some()).count()
    }

    /// Check if all pages are empty
    pub fn is_empty(&self) -> bool {
        self.inscribed_count() == 0
    }

    /// Get all pages as a slice (for UI display)
    pub fn pages(&self) -> &[Option<Page>] {
        &self.pages
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tome Errors
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TomeError {
    /// Page index is out of bounds
    IndexOutOfBounds { index: usize, capacity: usize },
}
