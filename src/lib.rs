//! A typed bitset container for Rust.
//!
//! # Example
//! ```
//! use typed_fixedbitset::TypedFixedBitSet;
//!
//! fn main() {
//!     let set = vec![S(0), S(2)]
//!         .into_iter()
//!         .collect::<TypedFixedBitSet<_>>();
//!
//!     assert!(set.contains(S(0)));
//!     assert!(!set.contains(S(1)));
//!     assert!(!set.is_empty());
//! }
//!
//! struct S(u32);
//!
//! impl From<S> for usize {
//!     fn from(s: S) -> Self {
//!         s.0 as usize
//!     }
//! }
//! ```
use fixedbitset::FixedBitSet;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Index;

/// `TypedFixedBitSet` is a simple fixed size set of bits that each can
/// be enabled (1 / **true**) or disabled (0 / **false**).
///
/// The bit set has a fixed capacity in terms of enabling bits (and the
/// capacity can grow using the `grow` method).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypedFixedBitSet<K> {
    _k: PhantomData<K>,
    is_empty: bool,
    set: FixedBitSet,
}

impl<K> Default for TypedFixedBitSet<K>
where
    K: Into<usize>,
{
    #[inline]
    fn default() -> Self {
        TypedFixedBitSet::with_capacity(0)
    }
}

impl<K> TypedFixedBitSet<K>
where
    K: Into<usize>,
{
    /// Create a new **TypedFixedBitSet** with a specific number of bits,
    /// all initially clear.
    #[inline]
    pub fn with_capacity(bits: usize) -> Self {
        TypedFixedBitSet {
            _k: PhantomData,
            is_empty: true,
            set: FixedBitSet::with_capacity(bits),
        }
    }

    /// Grow capacity to **bits**, all new bits initialized to zero
    #[inline]
    pub fn grow(&mut self, bits: usize) {
        self.set.grow(bits)
    }

    /// Return the length of the `FixedBitSet` in bits.
    #[inline]
    pub fn len(&self) -> usize {
        self.set.len()
    }

    /// Return **true** if the bit is enabled in the **FixedBitSet**,
    /// **false** otherwise.
    ///
    /// Note: k outside the capacity are always disabled.
    ///
    /// Note: Also available with index syntax: `bitset[k]`.
    #[inline]
    pub fn contains(&self, k: K) -> bool {
        self.set.contains(k.into())
    }

    /// Clear all bits.
    #[inline]
    pub fn clear(&mut self) {
        self.set.clear();
        self.is_empty = true;
    }

    /// Enable `bit`.
    ///
    /// **Panics** if **k** is out of bounds.
    #[inline]
    pub fn insert(&mut self, k: K) {
        self.set.insert(k.into());
        self.is_empty = false;
    }

    /// Return true if all the bit in the set are **false**.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Enable `bit`, and return its previous value.
    ///
    /// **Panics** if **k** is out of bounds.
    #[inline]
    pub fn put(&mut self, k: K) -> bool {
        let v = self.set.put(k.into());
        self.is_empty = false;
        v
    }

    /// **Panics** if **bit** is out of bounds.
    pub fn set(&mut self, k: K, enabled: bool) {
        self.set.set(k.into(), enabled);
        if enabled {
            self.is_empty = false;
        } else {
            self.is_empty = self.set.count_ones(..) == 0
        }
    }
}

impl<K> Clone for TypedFixedBitSet<K> {
    #[inline]
    fn clone(&self) -> Self {
        TypedFixedBitSet {
            set: self.set.clone(),
            is_empty: self.is_empty,
            _k: PhantomData,
        }
    }
}

/// Return **true** if the bit is enabled in the bitset,
/// or **false** otherwise.
///
/// Note: bits outside the capacity are always disabled, and thus
/// indexing a FixedBitSet will not panic.
impl<K> Index<K> for TypedFixedBitSet<K>
where
    K: Into<usize>,
{
    type Output = bool;

    #[inline]
    fn index(&self, k: K) -> &bool {
        self.set.index(k.into())
    }
}

/// Sets the bit at index **i** to **true** for each item **i** in the input **src**.
impl<K> Extend<K> for TypedFixedBitSet<K>
where
    K: Into<usize>,
{
    fn extend<I: IntoIterator<Item = K>>(&mut self, src: I) {
        let iter = src.into_iter();
        for i in iter {
            let i = i.into();
            if i >= self.len() {
                self.set.grow(i + 1);
            }
            self.is_empty = false;
            self.set.put(i);
        }
    }
}

/// Return a FixedBitSet containing bits set to **true** for every bit index in
/// the iterator, other bits are set to **false**.
impl<K> FromIterator<K> for TypedFixedBitSet<K>
where
    K: Into<usize>,
{
    fn from_iter<I: IntoIterator<Item = K>>(src: I) -> Self {
        let mut is_empty = true;

        let set = FixedBitSet::from_iter(src.into_iter().map(|i| {
            is_empty = false;
            i.into()
        }));

        TypedFixedBitSet {
            _k: PhantomData,
            is_empty,
            set,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, K> serde::de::Deserialize<'de> for TypedFixedBitSet<K>
where
    K: Into<usize> + serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        Ok(TypedFixedBitSet::from_iter(Vec::<K>::deserialize(
            deserializer,
        )?))
    }
}

#[cfg(feature = "serde")]
impl<K> serde::ser::Serialize for TypedFixedBitSet<K>
where
    K: From<usize> + serde::ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.0
            .ones()
            .map(|index| index.into())
            .collect::<Vec<K>>()
            .serialize(serializer)
    }
}
