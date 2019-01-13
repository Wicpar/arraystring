//! Draining iterator for [`ArrayString`]
//!
//! [`ArrayString`]: ../struct.ArrayString.html

use crate::{utils::Truncate, prelude::*};
use crate::core::fmt::{self, Debug, Formatter};
use crate::core::{cmp::Ordering, hash::Hash, hash::Hasher, iter::FusedIterator};

/// A draining iterator for [`ArrayString`].
///
/// Created through [`drain`]
///
/// [`ArrayString`]: ../struct.ArrayString.html
/// [`drain`]: ../struct.ArrayString.html#method.drain
#[derive(Clone, Default)]
pub struct Drain<S: Length>(pub(crate) ArrayString<S>, pub(crate) u8);

impl<SIZE> Debug for Drain<SIZE>
where
    SIZE: Length,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Drain")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

impl<SIZE> PartialEq for Drain<SIZE>
where
    SIZE: Length,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}
impl<SIZE: Length> Eq for Drain<SIZE> {}

impl<SIZE> Ord for Drain<SIZE>
where
    SIZE: Length,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<SIZE> PartialOrd for Drain<SIZE>
where
    SIZE: Length,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<SIZE> Hash for Drain<SIZE>
where
    SIZE: Length,
{
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher)
    }
}

impl<SIZE: Length + Copy> Copy for Drain<SIZE> where SIZE::Array: Copy {}

impl<S: Length> Drain<S> {
    /// Extracts string slice containing the remaining characters of `Drain`.
    #[inline]
    pub fn as_str(&self) -> &str {
        unsafe { self.0.as_str().get_unchecked(self.1.into()..) }
    }
}

impl<S: Length> Iterator for Drain<S> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .as_str()
            .get(self.1.into()..)
            .and_then(|s| s.chars().next())
            .map(|c| {
                self.1 = self.1.saturating_add(c.len_utf8().into_u8_lossy());
                c
            })
    }
}

impl<S: Length> DoubleEndedIterator for Drain<S> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<S: Length> FusedIterator for Drain<S> {}
