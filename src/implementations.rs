//! Trait implementations for `ArrayString` (that aren't for integration)

use crate::prelude::*;
use core::fmt::{self, Debug, Display, Formatter, Write};
use core::iter::FromIterator;
use core::ops::{Add, Deref, DerefMut, Index, IndexMut};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use core::str::{self, FromStr};
use core::{borrow::Borrow, borrow::BorrowMut, cmp::Ordering, hash::Hash, hash::Hasher};

impl<const N: usize> Default for ArrayString<N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> AsRef<str> for ArrayString<N> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> AsMut<str> for ArrayString<N> {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<const N: usize> AsRef<[u8]> for ArrayString<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a, const N: usize> From<&'a str> for ArrayString<N> {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_str_truncate(s)
    }
}

impl<const N: usize> FromStr for ArrayString<N> {
    type Err = OutOfBounds;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl<const N: usize> Debug for ArrayString<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ArrayString")
            .field("array", &self.as_str())
            .field("size", &self.size)
            .finish()
    }
}

impl<'a, 'b, const N: usize> PartialEq<str> for ArrayString<N> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl<const N: usize> Borrow<str> for ArrayString<N> {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> BorrowMut<str> for ArrayString<N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<const N: usize> Hash for ArrayString<N> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}

impl<const N: usize> PartialEq for ArrayString<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl<const N: usize> Eq for ArrayString<N> {}

impl<const N: usize> Ord for ArrayString<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const N: usize> PartialOrd for ArrayString<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, const N: usize> Add<&'a str> for ArrayString<N> {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &str) -> Self::Output {
        self.push_str(other);
        self
    }
}

impl<const N: usize> Write for ArrayString<N> {
    #[inline]
    fn write_str(&mut self, slice: &str) -> fmt::Result {
        self.try_push_str(slice).map_err(|_| fmt::Error)
    }
}

impl<const N: usize> Display for ArrayString<N> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> Deref for ArrayString<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<const N: usize> DerefMut for ArrayString<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<const N: usize> FromIterator<char> for ArrayString<N> {
    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
        Self::from_chars(iter)
    }
}

impl<'a, const N: usize> FromIterator<&'a str> for ArrayString<N> {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        Self::from_iterator(iter)
    }
}

impl<const N: usize> Extend<char> for ArrayString<N> {
    fn extend<I: IntoIterator<Item=char>>(&mut self, iterable: I) {
        self.push_str(Self::from_chars(iterable))
    }
}

impl<'a, const N: usize> Extend<&'a char> for ArrayString<N> {
    fn extend<I: IntoIterator<Item=&'a char>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<'a, const N: usize> Extend<&'a str> for ArrayString<N> {
    fn extend<I: IntoIterator<Item=&'a str>>(&mut self, iterable: I) {
        self.push_str(Self::from_iterator(iterable))
    }
}

impl<const N: usize> IndexMut<RangeFrom<u8>> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: RangeFrom<u8>) -> &mut str {
        let start = index.start as usize;
        self.as_mut_str().index_mut(RangeFrom { start })
    }
}

impl<const N: usize> IndexMut<RangeTo<u8>> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: RangeTo<u8>) -> &mut str {
        let end = index.end as usize;
        self.as_mut_str().index_mut(RangeTo { end })
    }
}

impl<const N: usize> IndexMut<RangeFull> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: RangeFull) -> &mut str {
        self.as_mut_str().index_mut(index)
    }
}

impl<const N: usize> IndexMut<Range<u8>> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: Range<u8>) -> &mut str {
        let (start, end) = (index.start as usize, index.end as usize);
        let range = Range { start, end };
        self.as_mut_str().index_mut(range)
    }
}

impl<const N: usize> IndexMut<RangeToInclusive<u8>> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: RangeToInclusive<u8>) -> &mut str {
        let end = index.end as usize;
        let range = RangeToInclusive { end };
        self.as_mut_str().index_mut(range)
    }
}

impl<const N: usize> IndexMut<RangeInclusive<u8>> for ArrayString<N> {
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<u8>) -> &mut str {
        let (start, end) = (*index.start() as usize, *index.end() as usize);
        let range = RangeInclusive::new(start, end);
        self.as_mut_str().index_mut(range)
    }
}

impl<const N: usize> Index<RangeFrom<u8>> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFrom<u8>) -> &Self::Output {
        let start = index.start as usize;
        self.as_str().index(RangeFrom { start })
    }
}

impl<const N: usize> Index<RangeTo<u8>> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeTo<u8>) -> &Self::Output {
        let end = index.end as usize;
        self.as_str().index(RangeTo { end })
    }
}

impl<const N: usize> Index<RangeFull> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeFull) -> &Self::Output {
        self.as_str().index(index)
    }
}

impl<const N: usize> Index<Range<u8>> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: Range<u8>) -> &Self::Output {
        let (start, end) = (index.start as usize, index.end as usize);
        self.as_str().index(Range { start, end })
    }
}

impl<const N: usize> Index<RangeToInclusive<u8>> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeToInclusive<u8>) -> &Self::Output {
        let end = index.end as usize;
        self.as_str().index(RangeToInclusive { end })
    }
}

impl<const N: usize> Index<RangeInclusive<u8>> for ArrayString<N> {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeInclusive<u8>) -> &Self::Output {
        let (start, end) = (*index.start() as usize, *index.end() as usize);
        let range = RangeInclusive::new(start, end);
        self.as_str().index(range)
    }
}
