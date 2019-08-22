use crate::constraint::{HasCharCount, HasElement, HasLength, IsEmptyValue};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

impl IsEmptyValue for String {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl IsEmptyValue for &str {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmptyValue for Vec<T> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmptyValue for &[T] {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmptyValue for HashSet<T> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> IsEmptyValue for HashMap<K, V> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> IsEmptyValue for Option<T>
where
    T: IsEmptyValue,
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

impl HasElement<String> for String {
    fn has_element(&self, element: &String) -> bool {
        self.contains(element)
    }
}

impl<V> HasElement<V> for HashSet<V>
where
    V: Eq + Hash,
{
    fn has_element(&self, element: &V) -> bool {
        self.contains(element)
    }
}

impl<K, V> HasElement<K> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn has_element(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}
