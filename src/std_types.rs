use crate::property::{HasCharCount, HasCheckedValue, HasEmptyValue, HasLength, HasMember};
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash};

impl HasCheckedValue for bool {
    fn is_checked_value(&self) -> bool {
        *self
    }
}

impl HasEmptyValue for String {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl HasEmptyValue for &str {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmptyValue for Vec<T> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmptyValue for &[T] {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T, S> HasEmptyValue for HashSet<T, S> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V, S> HasEmptyValue for HashMap<K, V, S> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmptyValue for Option<T>
where
    T: HasEmptyValue,
{
    fn is_empty_value(&self) -> bool {
        match self {
            Some(value) => value.is_empty_value(),
            None => true,
        }
    }
}

impl HasLength for String {
    fn length(&self) -> usize {
        self.len()
    }
}

impl HasLength for &str {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl HasCharCount for String {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl HasCharCount for &str {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl HasCharCount for Vec<char> {
    fn char_count(&self) -> usize {
        self.len()
    }
}

impl HasCharCount for &[char] {
    fn char_count(&self) -> usize {
        self.len()
    }
}

impl HasMember<String> for String {
    fn has_member(&self, element: &String) -> bool {
        self.contains(element)
    }
}

impl<V, S> HasMember<V> for HashSet<V, S>
where
    V: Eq + Hash,
    S: BuildHasher,
{
    fn has_member(&self, element: &V) -> bool {
        self.contains(element)
    }
}

impl<K, V, S> HasMember<K> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn has_member(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}
