pub trait MoveWithin {
    fn move_within(&mut self, from: usize, to: usize) -> bool;
}

impl<T> MoveWithin for Vec<T> {
    /// Move an element within a Vec
    ///
    /// returns `true` if from != to, indicating the Vec was modified
    ///
    /// # Panics
    ///
    /// Panics if the `from` or `to` indices are out of bounds.
    fn move_within(&mut self, from: usize, to: usize) -> bool {
        if from == to {
            false
        } else {
            let element = self.remove(from);
            self.insert(to, element);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indices_equal() {
        let mut v = vec!["a", "b"];

        let moved = v.move_within(0, 0);

        assert!(!moved);
        assert_eq!(vec!["a", "b"], v);
    }

    #[test]
    fn test_from_gt_to() {
        let mut v = vec!["a", "b", "c", "d"];

        let moved = v.move_within(3, 1);
        assert!(moved);
        assert_eq!(vec!["a", "d", "b", "c"], v);
    }

    #[test]
    fn test_from_lt_to() {
        let mut v = vec!["a", "b", "c", "d"];

        let moved = v.move_within(1, 3);
        assert!(moved);
        assert_eq!(vec!["a", "c", "d", "b"], v);
    }

    #[test]
    #[should_panic]
    fn panics_out_of_bounds() {
        let mut v = vec!["a", "b", "c", "d"];

        v.move_within(1, 4);
    }
}
