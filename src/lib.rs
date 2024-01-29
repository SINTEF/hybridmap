//! HybridMap is a Rust™ hybrid map implementation that uses a vector for small maps and a hash map overwise.
//!
//! As with most hybrid technologies, including two components instead of one is one too many. However, the hybrid solution can provide some value for specific use cases.
//!
//! HybridMap can be slightly faster for tiny maps, especially short-lived ones living on the memory stack, usually up to 16 entries and without too many lookups.
//!
//! ## Example
//!
//! HybridMap can be used like most other maps.
//!
//! ```rust
//! use hybridmap::HybridMap;
//!
//! let mut map = HybridMap::<i32, &str, 8>::new();
//! map.insert(1, "one");
//! map.insert(2, "two");
//!
//! assert_eq!(map.get(&1), Some(&"one"));
//! assert_eq!(map.len(), 2);
//! ```
//!
//! ## Why ?
//!
//! I started benchmarking tiny maps to check whether I should switch from HashMap to BTreeMap for my use case. I also had a naive Vec implementation that was surprisingly faster for my use case. Thus, I made this crate for fun.
//!
//! The energy savings this crate may bring probably do not compensate for the energy I used to boil water for my tea while implementing this crate. But it was fun.
use smallvec::SmallVec;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
enum InnerContainer<K, V, const N: usize> {
    // We use SmallVec for convenience, as it provides Vec-like ergonomics
    // while not using the memory heap.
    Vec(SmallVec<(K, V), N>),
    // We switch to the standard library HashMap when we reach the capacity
    HashMap(HashMap<K, V>),
}

/// A map that uses a `Vec` for small numbers of elements and a `HashMap` for
/// larger numbers of elements.
///
/// This is useful when you have a map that will usually have a small number of
/// elements, but may occasionally have a large number of elements.
///
/// The `N` type parameter specifies the maximum number of elements that can be
/// stored in the `Vec` before it is converted to a `HashMap`. The default value
/// is 16.
///
/// # Examples
///
/// ```
/// use hybridmap::HybridMap;
///
/// let mut map = HybridMap::<i32, &str, 8>::new();
/// map.insert(1, "one");
/// map.insert(2, "two");
///
/// assert_eq!(map.get(&1), Some(&"one"));
/// assert_eq!(map.len(), 2);
/// ```
///
#[derive(Clone, Debug)]
pub struct HybridMap<K, V, const N: usize = 8> {
    inner: InnerContainer<K, V, N>,
}

// Default trait.
impl<K, V, const N: usize> Default for HybridMap<K, V, N>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, const N: usize> HybridMap<K, V, N>
where
    K: Eq + Hash,
{
    /// Creates an empty `HybridMap`.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: InnerContainer::Vec(SmallVec::new()),
        }
    }

    /// Creates an empty `HybridMap` with the specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity <= N {
            Self {
                inner: InnerContainer::Vec(SmallVec::with_capacity(capacity)),
            }
        } else {
            Self {
                inner: InnerContainer::HashMap(HashMap::with_capacity(capacity)),
            }
        }
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        match &self.inner {
            InnerContainer::Vec(vec) => vec.len(),
            InnerContainer::HashMap(map) => map.len(),
        }
    }

    /// Returns `true` if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match &self.inner {
            InnerContainer::Vec(vec) => vec.is_empty(),
            InnerContainer::HashMap(map) => map.is_empty(),
        }
    }

    /// Get a reference to an element from the map.
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        match &self.inner {
            InnerContainer::Vec(vec) => vec
                .iter()
                .find_map(|(k, v)| if k == key { Some(v) } else { None }),
            InnerContainer::HashMap(map) => map.get(key),
        }
    }

    /// Get a mutable reference to an element from the map.
    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match &mut self.inner {
            InnerContainer::Vec(vec) => {
                vec.iter_mut()
                    .find_map(|(k, v)| if k == key { Some(v) } else { None })
            }
            InnerContainer::HashMap(map) => map.get_mut(key),
        }
    }

    /// Insert an element into the map.
    ///
    /// Returns the previous value if the key was already present.
    /// Returns `None` if the key was not present.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.inner {
            InnerContainer::Vec(vec) => {
                // Check if the vec contains the key already
                for (k, v) in vec.iter_mut() {
                    if k == &key {
                        let previous_value = std::mem::replace(v, value);
                        return Some(previous_value);
                    }
                }

                if vec.len() == N {
                    let mut map = HashMap::new();
                    for (k, v) in vec.drain(..) {
                        map.insert(k, v);
                    }
                    map.insert(key, value);
                    self.inner = InnerContainer::HashMap(map);
                    None
                } else {
                    vec.push((key, value));
                    None
                }
            }
            InnerContainer::HashMap(map) => map.insert(key, value),
        }
    }

    /// Remove an entry from the map by its key.
    /// returns the entry if it existed.
    #[inline]
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        match &mut self.inner {
            InnerContainer::Vec(vec) => vec
                .iter()
                .position(|(k, _)| k == key)
                .map(|index| vec.remove(index)),
            InnerContainer::HashMap(map) => map.remove_entry(key),
        }
    }

    /// Remove an entry from the map by its key.
    /// returns the value if it existed.
    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match &mut self.inner {
            InnerContainer::Vec(vec) => vec
                .iter()
                .position(|(k, _)| k == key)
                .map(|index| vec.remove(index).1),
            InnerContainer::HashMap(map) => map.remove(key),
        }
    }

    /// Clear the map, removing all elements.
    #[inline]
    pub fn clear(&mut self) {
        match &mut self.inner {
            InnerContainer::Vec(vec) => vec.clear(),
            InnerContainer::HashMap(map) => map.clear(),
        }
    }

    /// Returns an iterator over the entries of the map.
    #[inline]
    pub fn iter(&self) -> HybridMapIter<'_, K, V> {
        match &self.inner {
            InnerContainer::Vec(vec) => HybridMapIter::Vec(vec.iter()),
            InnerContainer::HashMap(map) => HybridMapIter::HashMap(map.iter()),
        }
    }

    /// Returns a mutable iterator over the entries of the map.
    #[inline]
    pub fn iter_mut(&mut self) -> HybridMapIterMut<'_, K, V> {
        match &mut self.inner {
            InnerContainer::Vec(vec) => HybridMapIterMut::Vec(vec.iter_mut()),
            InnerContainer::HashMap(map) => HybridMapIterMut::HashMap(map.iter_mut()),
        }
    }
}

/// An iterator over the entries of a `HybridMap`.
pub enum HybridMapIter<'a, K, V> {
    Vec(std::slice::Iter<'a, (K, V)>),
    HashMap(std::collections::hash_map::Iter<'a, K, V>),
}

impl<'a, K, V> Iterator for HybridMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridMapIter::Vec(iter) => iter.next().map(|(k, v)| (k, v)),
            HybridMapIter::HashMap(iter) => iter.next(),
        }
    }
}

/// A mutable iterator over the entries of a `HybridMap`.
pub enum HybridMapIterMut<'a, K, V> {
    Vec(std::slice::IterMut<'a, (K, V)>),
    HashMap(std::collections::hash_map::IterMut<'a, K, V>),
}

impl<'a, K, V> Iterator for HybridMapIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridMapIterMut::Vec(iter) => iter.next().map(|(ref mut k, ref mut v)| (&*k, &mut *v)),
            HybridMapIterMut::HashMap(iter) => iter.next(),
        }
    }
}

impl<K: Eq + Hash, V, const N: usize> IntoIterator for HybridMap<K, V, N> {
    type Item = (K, V);
    type IntoIter = HybridMapIntoIter<K, V, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self.inner {
            InnerContainer::Vec(vec) => HybridMapIntoIter::Vec(vec.into_iter()),
            InnerContainer::HashMap(map) => HybridMapIntoIter::HashMap(map.into_iter()),
        }
    }
}

/// A consuming iterator over the entries of a `HybridMap`.
pub enum HybridMapIntoIter<K, V, const N: usize> {
    Vec(smallvec::IntoIter<(K, V), N>),
    HashMap(std::collections::hash_map::IntoIter<K, V>),
}

impl<K, V, const N: usize> Iterator for HybridMapIntoIter<K, V, N> {
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            HybridMapIntoIter::Vec(iter) => iter.next(),
            HybridMapIntoIter::HashMap(iter) => iter.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a filled map
    fn filled_map(size: usize) -> HybridMap<i64, i64> {
        let mut map = HybridMap::<i64, i64>::new();
        for i in 0..(size as i64) {
            map.insert(i, i * 10);
        }
        map
    }

    #[test]
    fn new_map_is_empty() {
        let map: HybridMap<i64, i64> = HybridMap::new();
        assert!(map.get(&1).is_none());
    }

    #[test]
    fn with_capacity_initializes_correct_inner_container() {
        const TEST_THRESHOLD: usize = 16;
        let small_map = HybridMap::<i64, i64, TEST_THRESHOLD>::with_capacity(TEST_THRESHOLD - 1);
        match small_map.inner {
            InnerContainer::Vec(_) => {}
            _ => panic!("Should be a Vec"),
        }

        let large_map = HybridMap::<i64, i64, TEST_THRESHOLD>::with_capacity(TEST_THRESHOLD + 1);
        match large_map.inner {
            InnerContainer::HashMap(_) => {}
            _ => panic!("Should be a HashMap"),
        }
    }

    #[test]
    fn insert_and_get_work_correctly() {
        let mut map = filled_map(5);
        assert_eq!(map.get(&3), Some(&30));
        assert_eq!(map.insert(3, 35), Some(30));
        assert_eq!(map.get(&3), Some(&35));
    }

    #[test]
    fn insert_transitions_from_vec_to_hashmap() {
        const TEST_THRESHOLD: usize = 16;
        let mut map = filled_map(TEST_THRESHOLD);
        // This insert should trigger the transition
        map.insert(TEST_THRESHOLD as i64, TEST_THRESHOLD as i64 * 10);
        match map.inner {
            InnerContainer::HashMap(_) => {}
            _ => panic!("Should have transitioned to HashMap"),
        }
    }

    #[test]
    fn overwrite_existing_value() {
        const TEST_THRESHOLD: usize = 8;
        let mut map = HybridMap::<i32, i32, TEST_THRESHOLD>::new();
        assert_eq!(map.insert(1, 10), None); // First insert
        assert_eq!(map.insert(1, 20), Some(10)); // Overwrite
        assert_eq!(map.get(&1), Some(&20)); // Check new value
    }

    #[test]
    fn remove_and_reinsert_key() {
        let mut map = filled_map(5);
        assert_eq!(map.insert(3, 35), Some(30)); // Overwrite
        assert_eq!(map.insert(3, 40), Some(35)); // Overwrite again
        assert_eq!(map.get(&3), Some(&40)); // Check new value
    }

    #[test]
    fn capacity_expansion() {
        const TEST_THRESHOLD: usize = 8;
        let mut map = HybridMap::<i32, i32, TEST_THRESHOLD>::with_capacity(5);
        for i in 0..15 {
            map.insert(i, i * 10);
        }
        assert_eq!(map.get(&10), Some(&100));
        assert!(matches!(map.inner, InnerContainer::HashMap(_)));
    }

    #[test]
    fn query_non_existent_key() {
        let map = filled_map(5);
        assert_eq!(map.get(&10), None);
    }

    #[test]
    fn stress_test_large_insertions() {
        const TEST_THRESHOLD: usize = 8;
        let mut map = HybridMap::<i32, i32, TEST_THRESHOLD>::new();
        for i in 0..1000 {
            map.insert(i, i * 10);
        }
        for i in 0..1000 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    pub fn test_len() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        assert_eq!(map.len(), 0);
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        assert_eq!(map.len(), 3);
        map.insert(4, 40);
        assert_eq!(map.len(), 4);
    }

    #[test]
    pub fn test_default() {
        let map = HybridMap::<i32, i32, 3>::default();
        assert_eq!(map.len(), 0);
    }

    #[test]
    pub fn test_clone() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        let map2 = map.clone();
        assert_eq!(map2.len(), 3);
        assert_eq!(map2.get(&1), Some(&10));
        assert_eq!(map2.get(&2), Some(&20));
        assert_eq!(map2.get(&3), Some(&30));
        map.insert(4, 40);
        assert_eq!(map2.len(), 3);
        let map3 = map.clone();
        assert_eq!(map3.len(), 4);
    }

    #[test]
    fn test_is_empty() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        assert!(map.is_empty());
        map.insert(1, 10);
        assert!(!map.is_empty());
        map.insert(2, 20);
        map.insert(3, 30);
        map.insert(4, 40);
        assert!(!map.is_empty());
    }

    #[test]
    fn test_get_mut() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        assert_eq!(map.get_mut(&1), Some(&mut 10));
        let entry = map.get_mut(&1).unwrap();
        *entry = 20;
        assert_eq!(map.get(&1), Some(&20));

        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map.insert(4, 40);
        let entry = map.get_mut(&1).unwrap();
        *entry = 30;
        assert_eq!(map.get(&1), Some(&30));
    }

    #[test]
    fn test_remove_entry() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        assert_eq!(map.remove_entry(&2), Some((2, 20)));
        assert!(map.len() == 2);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.remove_entry(&2), None);
        map.insert(4, 40);
        map.insert(5, 50);
        assert_eq!(map.remove_entry(&12), None);
        assert_eq!(map.remove_entry(&3), Some((3, 30)));
        assert!(map.len() == 3);
    }

    #[test]
    fn test_remove() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        assert_eq!(map.remove(&2), Some(20));
        assert!(map.len() == 2);
        assert_eq!(map.get(&2), None);
        assert_eq!(map.remove(&2), None);
        map.insert(4, 40);
        map.insert(5, 50);
        assert_eq!(map.remove(&12), None);
        assert_eq!(map.remove(&3), Some(30));
        assert!(map.len() == 3);
    }

    #[test]
    fn test_clear() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.get(&1), None);
        map.insert(2, 20);
        map.insert(3, 30);
        map.insert(4, 40);
        map.insert(5, 50);
        assert_eq!(map.len(), 4);
        map.clear();
        assert!(map.is_empty());
        assert_eq!(map.get(&4), None);
    }

    #[test]
    fn test_iter() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        let mut iter = map.iter();
        // Ordered
        assert_eq!(iter.next(), Some((&1, &10)));
        assert_eq!(iter.next(), Some((&2, &20)));
        assert_eq!(iter.next(), Some((&3, &30)));
        assert_eq!(iter.next(), None);
        map.insert(4, 40);
        let iter = map.iter();
        let vec = iter.collect::<Vec<_>>();
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn test_iter_mut() {
        // Add +1
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        for (_, v) in map.iter_mut() {
            *v += 1;
        }
        assert_eq!(map.get(&1), Some(&11));
        assert_eq!(map.get(&2), Some(&21));
        assert_eq!(map.get(&3), Some(&31));

        map.insert(4, 40);
        for (_, v) in map.iter_mut() {
            *v += 1;
        }

        assert_eq!(map.get(&1), Some(&12));
        assert_eq!(map.get(&4), Some(&41));
    }

    #[test]
    fn test_into_iter() {
        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        let vec: Vec<_> = map.into_iter().collect();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], (1, 10));
        assert_eq!(vec[1], (2, 20));

        let mut map = HybridMap::<i32, i32, 3>::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map.insert(4, 40);
        let vec: Vec<_> = map.into_iter().collect();
        assert_eq!(vec.len(), 4);
        let sum = vec.iter().fold(0, |acc, (_, v)| acc + v);
        assert_eq!(sum, 100);
    }
}
