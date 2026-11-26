//! Bounded generic undo/redo snapshot history stack.

/// Maintains undo and redo snapshots of state `T`.
#[derive(Debug, Clone)]
pub struct HistoryStack<T: Clone> {
    undo_stack: Vec<T>,
    redo_stack: Vec<T>,
    max_entries: usize,
}

impl<T: Clone> HistoryStack<T> {
    /// Creates a new history stack with the given maximum snapshot capacity.
    pub fn new(max_entries: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_entries: max_entries.max(1),
        }
    }

    /// Pushes a snapshot before a mutation and clears the redo stack.
    pub fn push(&mut self, snapshot: T) {
        if self.undo_stack.len() >= self.max_entries {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(snapshot);
        self.redo_stack.clear();
    }

    /// Restores the previous snapshot, pushing `current` onto the redo stack.
    pub fn undo(&mut self, current: &T) -> Option<T> {
        let prev = self.undo_stack.pop()?;
        if self.redo_stack.len() >= self.max_entries {
            self.redo_stack.remove(0);
        }
        self.redo_stack.push(current.clone());
        Some(prev)
    }

    /// Reapplies a previously undone snapshot, pushing `current` onto the undo stack.
    pub fn redo(&mut self, current: &T) -> Option<T> {
        let next = self.redo_stack.pop()?;
        if self.undo_stack.len() >= self.max_entries {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(current.clone());
        Some(next)
    }

    /// Returns true if an undo step is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if a redo step is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears all undo and redo entries.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Returns `(undo_count, redo_count)`.
    pub fn counts(&self) -> (usize, usize) {
        (self.undo_stack.len(), self.redo_stack.len())
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryStack;

    #[test]
    fn undo_redo_round_trip_and_clear_redo() {
        let mut hist = HistoryStack::new(10);
        let mut state = 1;

        hist.push(state);
        state = 2;
        hist.push(state);
        state = 3;

        assert!(hist.can_undo());
        assert!(!hist.can_redo());

        state = hist.undo(&state).unwrap();
        assert_eq!(state, 2);
        assert!(hist.can_redo());

        state = hist.undo(&state).unwrap();
        assert_eq!(state, 1);
        assert!(!hist.can_undo());

        state = hist.redo(&state).unwrap();
        assert_eq!(state, 2);

        // New edit clears redo stack.
        hist.push(state);
        state = 99;
        assert!(!hist.can_redo());
        assert_eq!(hist.undo(&state).unwrap(), 2);
    }

    #[test]
    fn respects_max_capacity() {
        let mut hist = HistoryStack::new(2);
        hist.push(10);
        hist.push(20);
        hist.push(30);
        assert_eq!(hist.counts(), (2, 0));
        assert_eq!(hist.undo(&40), Some(30));
        assert_eq!(hist.undo(&30), Some(20));
        assert_eq!(hist.undo(&20), None);
    }
}
