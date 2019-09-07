use crate::property::{HasCharCount, HasCheckedValue, HasEmptyValue, HasLength, HasMember};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
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

impl<T> HasEmptyValue for VecDeque<T> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmptyValue for LinkedList<T> {
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

impl<T> HasEmptyValue for BTreeSet<T> {
    fn is_empty_value(&self) -> bool {
        self.is_empty()
    }
}

impl<K, V> HasEmptyValue for BTreeMap<K, V> {
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

impl<T> HasLength for VecDeque<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for LinkedList<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for BTreeSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<K, V> HasLength for BTreeMap<K, V> {
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

impl<T> HasMember<T> for VecDeque<T>
where
    T: PartialEq,
{
    fn has_member(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T> HasMember<T> for LinkedList<T>
where
    T: PartialEq,
{
    fn has_member(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<T, S> HasMember<T> for HashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn has_member(&self, element: &T) -> bool {
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

impl<T> HasMember<T> for BTreeSet<T>
where
    T: Ord,
{
    fn has_member(&self, element: &T) -> bool {
        self.contains(element)
    }
}

impl<K, V> HasMember<K> for BTreeMap<K, V>
where
    K: Ord,
{
    fn has_member(&self, element: &K) -> bool {
        self.contains_key(element)
    }
}

#[cfg(not(feature = "num-traits"))]
mod num {
    use crate::property::HasZeroValue;

    impl HasZeroValue for i8 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for i16 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for i32 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for i64 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for i128 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for u8 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for u16 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for u32 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for u64 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for u128 {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for isize {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for usize {
        fn is_zero_value(&self) -> bool {
            *self == 0
        }
    }

    impl HasZeroValue for f32 {
        fn is_zero_value(&self) -> bool {
            *self == 0.
        }
    }

    impl HasZeroValue for f64 {
        fn is_zero_value(&self) -> bool {
            *self == 0.
        }
    }
}
