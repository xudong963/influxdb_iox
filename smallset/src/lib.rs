//! Memory-efficient set that is read often and modified rarely.
#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]

/// Set that is backed by a sorted vector.
///
/// Elements stored in this set are immutable.
///
/// The vector is always allocated "at capacity".
#[derive(Clone)]
pub struct SmallSet<T>
where
    T: Ord,
{
    v: Vec<T>,
}

impl<T> SmallSet<T>
where
    T: Ord,
{
    /// Create new empty set.
    pub fn new() -> Self {
        Self {
            v: Vec::with_capacity(0),
        }
    }

    /// Return number of stored entries.
    ///
    /// The number of stored entries and the capacity are identical.
    pub fn len(&self) -> usize {
        self.v.len()
    }

    /// Returns `true` if the set does not contain any elements.
    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    /// Insert element into the set.
    ///
    /// Returns `true` if the element was not yet part of the set.
    pub fn insert(&mut self, e: T) -> bool {
        if let Err(idx) = self.v.binary_search(&e) {
            let mut v_new = Vec::with_capacity(self.v.len() + 1);
            let mut v_tail = self.v.split_off(idx);
            v_new.append(&mut self.v);
            v_new.push(e);
            v_new.append(&mut v_tail);
            self.v = v_new;

            true
        } else {
            false
        }
    }

    /// Iterator over elements, by ref.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.v.iter()
    }
}

impl<T> Default for SmallSet<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for SmallSet<T>
where
    T: Ord,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a SmallSet<T>
where
    T: Ord,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.iter()
    }
}

impl<T> std::ops::Deref for SmallSet<T>
where
    T: Ord,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

impl<T> std::fmt::Debug for SmallSet<T>
where
    T: std::fmt::Debug + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl<T> From<Vec<T>> for SmallSet<T>
where
    T: Ord,
{
    fn from(mut v: Vec<T>) -> Self {
        v.sort();
        v.dedup();
        v.shrink_to_fit();
        Self { v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_logic() {
        // empty vec
        let mut s = SmallSet::<u8>::new();
        assert_eq!(s.as_ref(), &[]);

        // insert 5
        assert!(s.insert(5));
        assert_eq!(s.as_ref(), &[5]);

        // insert 5 again
        assert!(!s.insert(5));
        assert_eq!(s.as_ref(), &[5]);

        // insert 2
        assert!(s.insert(2));
        assert_eq!(s.as_ref(), &[2, 5]);

        // insert 8
        assert!(s.insert(8));
        assert_eq!(s.as_ref(), &[2, 5, 8]);

        // insert 5 again
        assert!(!s.insert(5));
        assert_eq!(s.as_ref(), &[2, 5, 8]);

        // insert 2 again
        assert!(!s.insert(2));
        assert_eq!(s.as_ref(), &[2, 5, 8]);

        // insert 8 again
        assert!(!s.insert(8));
        assert_eq!(s.as_ref(), &[2, 5, 8]);

        // insert 3
        assert!(s.insert(3));
        assert_eq!(s.as_ref(), &[2, 3, 5, 8]);
    }

    #[test]
    fn test_len() {
        // empty vec
        let mut s = SmallSet::<u8>::new();
        assert_eq!(s.len(), 0);

        // insert 5
        s.insert(5);
        assert_eq!(s.len(), 1);

        // insert 2
        s.insert(2);
        assert_eq!(s.len(), 2);

        // insert 5 again
        s.insert(5);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        // empty vec
        let mut s = SmallSet::<u8>::new();
        assert!(s.is_empty());

        // insert 5
        s.insert(5);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_default() {
        // empty vec
        let s = SmallSet::<u8>::default();
        assert!(s.is_empty());
    }

    #[test]
    fn test_debug() {
        // empty vec
        let mut s = SmallSet::<u8>::new();
        assert_eq!(format!("{:?}", s), "{}");

        // insert 5
        s.insert(5);
        assert_eq!(format!("{:?}", s), "{5}");

        // insert 2
        s.insert(2);
        assert_eq!(format!("{:?}", s), "{2, 5}");

        // insert 5 again
        s.insert(5);
        assert_eq!(format!("{:?}", s), "{2, 5}");
    }

    #[test]
    fn test_clone() {
        let mut s1 = SmallSet::<u8>::new();
        s1.insert(5);

        let mut s2 = s1.clone();
        s2.insert(2);

        assert_eq!(s1.as_ref(), &[5]);
        assert_eq!(s2.as_ref(), &[2, 5]);
    }

    #[test]
    fn test_memory_usage() {
        let mut s = SmallSet::<u8>::new();
        assert_eq!(s.v.capacity(), 0);

        s.insert(5);
        assert_eq!(s.v.capacity(), 1);

        s.insert(2);
        assert_eq!(s.v.capacity(), 2);

        s.insert(8);
        assert_eq!(s.v.capacity(), 3);
    }

    #[test]
    fn test_from_vec() {
        let mut v = Vec::<u8>::with_capacity(10);
        v.push(5);
        v.push(2);
        v.push(8);
        v.push(2);

        let s = SmallSet::from(v);
        assert_eq!(s.as_ref(), &[2, 5, 8]);
        assert_eq!(s.v.capacity(), 3);
    }
}
