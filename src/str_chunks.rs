/*
 * --------------------
 * THIS FILE IS LICENSED UNDER THE FOLLOWING TERMS
 *
 * all rights reserved. be gay, do crime
 *
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */
#![no_std]

//! # str chunks
//! implements char-wise chunked iteration of str
//!
//! the methods [`str_chunks`], [`str_chunks_exact`], [`str_rchunks`], and [`str_rchunks_exact`]
//! behave like the similarly named methods on slice,
//! but return string slices that are `chunk_size` chars long.
//! take note: these slices are not necessarily `chunk_size` *bytes* long. `chunk.len() != chunk_size`
//!
//! import [`ImplStrChunks`] to get methods on [`&str`]
//!
//! For [`DoubleEndedIterator`] support, use the `reversable` method.
//! This is not a trivial operation, and it is recommended to avoid it if possible.
//!
//! [`&str`]: str
//! [`DoubleEndedIterator`]: core::iter::DoubleEndedIterator
//! [`str_chunks`]: ImplStrChunks::str_chunks
//! [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
//! [`str_rchunks`]: ImplStrChunks::str_rchunks
//! [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact

use core::{
    iter::{once, Chain, Once},
    marker::PhantomData,
    num::NonZeroUsize,
    str::CharIndices,
};

/// Trait implemented on &str to provide convenience methods.
///
/// Import this!
///
/// # Example
/// ```
/// use str_chunks::ImplStrChunks;
/// let s = "lorem";
/// let mut iter = s.str_chunks(2);
/// assert_eq!(iter.next(), Some("lo"));
/// assert_eq!(iter.next(), Some("re"));
/// assert_eq!(iter.next(), Some("m"));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remaining(), "");
/// ```
pub trait ImplStrChunks {
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the beginning of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last chunk will not have char-length `chunk_size`.
    ///
    /// See [`str_chunks_exact`] for a variant of this iterator that returns chunks of always
    /// exactly `chunk_size` elements, and [`str_rchunks`] for the same iterator but starting at
    /// the end of the str.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use str_chunks::*;
    /// let s = "lorem";
    /// let mut iter = s.str_chunks(2);
    /// assert_eq!(iter.next(), Some("lo"));
    /// assert_eq!(iter.next(), Some("re"));
    /// assert_eq!(iter.next(), Some("m"));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remaining(), "");
    /// ```
    /// [`str_chunks`]: ImplStrChunks::str_chunks
    /// [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
    /// [`str_rchunks`]: ImplStrChunks::str_rchunks
    /// [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact
    fn str_chunks(&self, chunk_size: usize) -> StrChunks;
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the beginning of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last up to `chunk_size-1` chars will be omitted and can be retrieved from the
    /// [`remaining`] function of the iterator after the iterator has returned None.
    ///
    /// See [`str_chunks`] for a variant of this iterator that also returns the remainder as a
    /// smaller chunk.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use str_chunks::*;
    /// let s = "lorem";
    /// let mut iter = s.str_chunks_exact(2);
    /// assert_eq!(iter.next(), Some("lo"));
    /// assert_eq!(iter.next(), Some("re"));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remaining(), "m");
    /// ```
    /// [`str_chunks`]: ImplStrChunks::str_chunks
    /// [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
    /// [`str_rchunks`]: ImplStrChunks::str_rchunks
    /// [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact
    /// [`remaining`]: StrChunksExact::remaining
    fn str_chunks_exact(&self, chunk_size: usize) -> StrChunksExact;
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the end of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last chunk will not have char-length `chunk_size`.
    ///
    /// See [`str_rchunks_exact`] for a variant of this iterator that returns chunks of always
    /// exactly `chunk_size` elements, and [`str_chunks`] for the same iterator but starting at the
    /// beginning of the str.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use str_chunks::*;
    /// let s = "lorem";
    /// let mut iter = s.str_rchunks(2);
    /// assert_eq!(iter.next(), Some("em"));
    /// assert_eq!(iter.next(), Some("or"));
    /// assert_eq!(iter.next(), Some("l"));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remaining(), "");
    /// ```
    /// [`str_chunks`]: ImplStrChunks::str_chunks
    /// [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
    /// [`str_rchunks`]: ImplStrChunks::str_rchunks
    /// [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact
    fn str_rchunks(&self, chunk_size: usize) -> StrRChunks;
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the end of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last up to `chunk_size-1` chars will be omitted and can be retrieved from the
    /// [`remaining`] function of the iterator after the iterator has
    /// returned None.
    ///
    /// See [`str_rchunks`] for a variant of this iterator that also returns the remainder as a
    /// smaller chunk, and [`str_chunks_exact`] for the same iterator but starting at the beginning
    /// of the str.
    ///
    /// # Panics
    /// Panics if `chunk_size` is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use str_chunks::*;
    /// let s = "lorem";
    /// let mut iter = s.str_rchunks_exact(2);
    /// assert_eq!(iter.next(), Some("em"));
    /// assert_eq!(iter.next(), Some("or"));
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.remaining(), "l");
    /// ```
    /// [`str_chunks`]: ImplStrChunks::str_chunks
    /// [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
    /// [`str_rchunks`]: ImplStrChunks::str_rchunks
    /// [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact
    /// [`remaining`]: StrChunksExact::remaining
    fn str_rchunks_exact(&self, chunk_size: usize) -> StrRChunksExact;
}
impl ImplStrChunks for &str {
    fn str_chunks(&self, chunk_size: usize) -> StrChunks {
        assert!(chunk_size != 0, "chunk size must be non-zero");
        // UNWRAP: already checking chunk_size is non-zero, unwrap will be optimized out
        StrChunks::new(self, chunk_size.try_into().unwrap())
    }
    fn str_chunks_exact(&self, chunk_size: usize) -> StrChunksExact {
        assert!(chunk_size != 0, "chunk size must be non-zero");
        // UNWRAP: already checking chunk_size is non-zero, unwrap will be optimized out
        StrChunksExact::new(self, chunk_size.try_into().unwrap())
    }
    fn str_rchunks(&self, chunk_size: usize) -> StrRChunks {
        assert!(chunk_size != 0, "chunk size must be non-zero");
        // UNWRAP: already checking chunk_size is non-zero, unwrap will be optimized out
        StrRChunks::new(self, chunk_size.try_into().unwrap())
    }
    fn str_rchunks_exact(&self, chunk_size: usize) -> StrRChunksExact {
        assert!(chunk_size != 0, "chunk size must be non-zero");
        // UNWRAP: already checking chunk_size is non-zero, unwrap will be optimized out
        StrRChunksExact::new(self, chunk_size.try_into().unwrap())
    }
}

#[allow(clippy::doc_markdown)]
/// str::char_indices does not include s.len() as an index.
/// the length is, however, a valid index for str::split_at.
/// this iterator adds the length of the str to the end of the char_indices
fn valid_split_points(s: &str) -> Chain<CharIndices, Once<(usize, char)>> {
    s.char_indices()
        .chain(once((s.len(), char::REPLACEMENT_CHARACTER)))
}

/// An iterator over a str in (non-overlapping) chunks (`chunk_size` chars at a time) starting at
/// the beginning of the str.
///
/// When the str len (in characters) is not evenly divided by the chunk size, the last str of
/// the iteration will be the remainder.
///
/// This struct is created by the [`str_chunks`] method on `str`
///
/// # Example
///
/// ```
/// use str_chunks::*;
/// let s = "lorem";
/// let mut iter = s.str_chunks(2);
/// assert_eq!(iter.next(), Some("lo"));
/// assert_eq!(iter.next(), Some("re"));
/// assert_eq!(iter.next(), Some("m"));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remaining(), "");
/// ```
///
/// [`str_chunks`]: ImplStrChunks::str_chunks
pub struct StrChunks<'s> {
    s: &'s str,
    // 0 chunk_size makes no sense
    chunk_size: NonZeroUsize,
}

impl<'s> StrChunks<'s> {
    fn new(s: &'s str, chunk_size: NonZeroUsize) -> Self {
        Self { s, chunk_size }
    }
    #[must_use]
    /// Returns the remaining string slice that has not yet been iterated over.
    pub fn remaining(&self) -> &'s str {
        self.s
    }
    #[must_use]
    /// Makes this iterator reversible.
    ///
    /// Reversing these iterators requires precomputing the number of characters in the string
    /// so the remainder can be split off in advance. This is not a trivial operation, as the only
    /// way to get the character is to iterate over them all.
    ///
    /// Keep in mind that because of remainders, reversing a [`StrChunks`] will not necessarily yield
    /// the same items as a corresponding [`StrRChunks`].
    pub fn reversable(self) -> DoubleEnded<'s, Self> {
        DoubleEnded::<'s, Self>::new(self)
    }
}

impl<'s> Iterator for StrChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let (index, _) = valid_split_points(self.s)
            .nth(self.chunk_size.get())
            // SAFETY: the end of a str is a valid byte offset, and is the boundary of a codepoint
            // by definition of a str.
            .unwrap_or((self.s.len(), char::REPLACEMENT_CHARACTER));
        // SAFETY: indices from valid_split_points are valid byte offsets on the boundary of a codepoint
        let (chunk, rest) = unsafe { self.s.split_at_checked(index).unwrap_unchecked() };
        self.s = rest;
        // chunk is empty if index is 0 and (no remainder or remainder was already returned)
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}

/// An iterator over a str in (non-overlapping) chunks (`chunk_size` chars at a time) starting at
/// the beginning of the str.
///
/// When the str len (in characters) is not evenly divided by the chunk size, the last up to
/// `chunk_size-1` chars can will be omitted but can be retrieved with the [`remaining`] function
/// after the iteration has returned None.
///
/// This struct is created by the [`str_chunks_exact`] method on `str`
///
/// # Example
///
/// ```
/// use str_chunks::*;
/// let s = "lorem";
/// let mut iter = s.str_chunks_exact(2);
/// assert_eq!(iter.next(), Some("lo"));
/// assert_eq!(iter.next(), Some("re"));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remaining(), "m");
/// ```
///
/// [`str_chunks_exact`]: ImplStrChunks::str_chunks_exact
/// [`remaining`]: StrChunksExact::remaining
pub struct StrChunksExact<'s> {
    s: &'s str,
    // 0 chunk_size makes no sense
    chunk_size: NonZeroUsize,
}

impl<'s> StrChunksExact<'s> {
    fn new(s: &'s str, chunk_size: NonZeroUsize) -> Self {
        Self { s, chunk_size }
    }
    #[must_use]
    /// Returns the remaining string slice that has not yet been iterated over.
    /// If [`Self::next`] has returned None, this is the remainder, and is at most N-1 chars long.
    pub fn remaining(&self) -> &'s str {
        self.s
    }
    #[must_use]
    /// Makes this iterator reversible.
    ///
    /// Reversing these iterators requires precomputing the number of characters in the string
    /// so the remainder can be split off in advance. This is not a trivial operation, as the only
    /// way to get the character is to iterate over them all.
    ///
    /// Keep in mind that because of remainders, reversing a [`StrChunks`] will not necessarily yield
    /// the same items as a corresponding [`StrRChunks`].
    pub fn reversable(self) -> DoubleEnded<'s, Self> {
        DoubleEnded::<'s, Self>::new(self)
    }
}

impl<'s> Iterator for StrChunksExact<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        // will short-circuit None if s.len() < chunk_size, leaving the remainder in s
        let (index, _) = valid_split_points(self.s).nth(self.chunk_size.get())?;

        // SAFETY: indices from valid_split_points are valid byte offsets on the boundary of a codepoint
        let (chunk, rest) = unsafe { self.s.split_at_checked(index).unwrap_unchecked() };

        self.s = rest;
        Some(chunk)
    }
}

/// An iterator over a str in (non-overlapping) chunks (`chunk_size` chars at a time) starting at
/// the end of the str.
///
/// When the str len (in characters) is not evenly divided by the chunk size, the last str of
/// the iteration will be the remainder.
///
/// This struct is created by the [`str_rchunks`] method on `str`
///
/// # Example
///
/// ```
/// use str_chunks::*;
/// let s = "lorem";
/// let mut iter = s.str_rchunks(2);
/// assert_eq!(iter.next(), Some("em"));
/// assert_eq!(iter.next(), Some("or"));
/// assert_eq!(iter.next(), Some("l"));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remaining(), "");
/// ```
///
/// [`str_rchunks`]: ImplStrChunks::str_rchunks
pub struct StrRChunks<'s> {
    s: &'s str,
    // 0 chunk_size makes no sense
    chunk_size: NonZeroUsize,
}

impl<'s> StrRChunks<'s> {
    fn new(s: &'s str, chunk_size: NonZeroUsize) -> Self {
        Self { s, chunk_size }
    }
    #[must_use]
    /// Returns the remaining string slice that has not yet been iterated over.
    pub fn remaining(&self) -> &'s str {
        self.s
    }
    #[must_use]
    /// Makes this iterator reversible.
    ///
    /// Reversing these iterators requires precomputing the number of characters in the string
    /// so the remainder can be split off in advance. This is not a trivial operation, as the only
    /// way to get the character is to iterate over them all.
    ///
    /// Keep in mind that because of remainders, reversing a [`StrChunks`] will not necessarily yield
    /// the same items as a corresponding [`StrRChunks`].
    pub fn reversable(self) -> DoubleEnded<'s, Self> {
        DoubleEnded::<'s, Self>::new(self)
    }
}

impl<'s> Iterator for StrRChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let (index, _) = valid_split_points(self.s)
            .nth_back(self.chunk_size.get())
            // SAFETY: the start of a str is a valid byte offset, and is the boundary of a codepoint
            // by definition of a str.
            .unwrap_or((0, char::REPLACEMENT_CHARACTER));
        // SAFETY: indices from valid_split_points are valid byte offsets on the boundary of a codepoint
        let (rest, chunk) = unsafe { self.s.split_at_checked(index).unwrap_unchecked() };
        self.s = rest;
        // chunk is empty if index is 0 and (no remainder or remainder was already returned)
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}

/// An iterator over a str in (non-overlapping) chunks (`chunk_size` chars at a time) starting at
/// the end of the str.
///
/// When the str len (in characters) is not evenly divided by the chunk size, the last up to
/// `chunk_size-1` chars can will be omitted but can be retrieved with the [`remaining`] function
/// after the iteration has returned None.
///
/// This struct is created by the [`str_rchunks_exact`] method on `str`
///
/// # Example
///
/// ```
/// use str_chunks::*;
/// let s = "lorem";
/// let mut iter = s.str_rchunks_exact(2);
/// assert_eq!(iter.next(), Some("em"));
/// assert_eq!(iter.next(), Some("or"));
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remaining(), "l");
/// ```
///
/// [`str_rchunks_exact`]: ImplStrChunks::str_rchunks_exact
/// [`remaining`]: StrRChunksExact::remaining
pub struct StrRChunksExact<'s> {
    s: &'s str,
    // 0 chunk_size makes no sense
    chunk_size: NonZeroUsize,
}

impl<'s> StrRChunksExact<'s> {
    fn new(s: &'s str, chunk_size: NonZeroUsize) -> Self {
        Self { s, chunk_size }
    }
    #[must_use]
    /// Returns the remaining string slice that has not yet been iterated over.
    /// If [`Self::next`] has returned None, this is the remainder, and is at most N-1 chars long.
    pub fn remaining(&self) -> &'s str {
        self.s
    }
    #[must_use]
    /// Makes this iterator reversible.
    ///
    /// Reversing these iterators requires precomputing the number of characters in the string
    /// so the remainder can be split off in advance. This is not a trivial operation, as the only
    /// way to get the character is to iterate over them all.
    ///
    /// Keep in mind that because of remainders, reversing a [`StrChunks`] will not necessarily yield
    /// the same items as a corresponding [`StrRChunks`].
    pub fn reversable(self) -> DoubleEnded<'s, Self> {
        DoubleEnded::<'s, Self>::new(self)
    }
}

impl<'s> Iterator for StrRChunksExact<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        // will short-circuit None if s.len() < chunk_size, leaving the remainder in s
        let (index, _) = valid_split_points(self.s).nth_back(self.chunk_size.get())?;

        // SAFETY: indices from valid_split_points are valid byte offsets on the boundary of a codepoint
        let (rest, chunk) = unsafe { self.s.split_at_checked(index).unwrap_unchecked() };

        self.s = rest;
        Some(chunk)
    }
}

/// A double-ended variant of a strchunks variant.
/// This struct is created by the [`reversable`] method on any strchunks variant.
///
/// Behaves exactly the same as the variant it was produced from,
/// but implements [`DoubleEndedIterator`], letting you iterate from both ends,
/// as well as reverse the iterator.
///
/// When the str len (in characters) is not evenly divided by the chunk size, the last up to
/// `chunk_size-1` chars is the "remainder".
/// For Exact iterators, the remainder will be omitted, but can be retrieved with the [`remainder`] function
/// at any time.
/// For non-Exact iterators, the remainder is the last element to be yielded from the front,
/// or the first to be yielded from the back.
///
///
/// # Example
///
/// ```
/// use str_chunks::*;
/// let s = "lorem";
/// let mut iter = s.str_chunks_exact(2).reversable();
/// assert_eq!(iter.next_back(), Some("re"));
/// assert_eq!(iter.next_back(), Some("lo"));
/// assert_eq!(iter.next_back(), None);
/// assert_eq!(iter.next(), None);
/// assert_eq!(iter.remainder(), "m");
///
/// let mut iter = s.str_chunks(2).reversable();
/// assert_eq!(iter.next_back(), Some("m"));
/// assert_eq!(iter.next_back(), Some("re"));
/// assert_eq!(iter.next_back(), Some("lo"));
/// assert_eq!(iter.next_back(), None);
/// assert_eq!(iter.next(), None);
/// ```
///
/// [`reversable`]: StrChunks::reversable
/// [`DoubleEndedIterator`]: core::iter::DoubleEndedIterator
/// [`remainder`]: DoubleEnded::remainder
pub struct DoubleEnded<'s, T> {
    s: &'s str,
    chunk_size: NonZeroUsize,
    remainder: &'s str,
    _inner: PhantomData<T>,
}
impl<'s, T> DoubleEnded<'s, T> {
    #[must_use]
    /// Returns the remaining string slice that has not yet been iterated over,
    /// not including the remainder.
    ///
    /// Note this is NOT the same as the `remainder` function available on Exact variants.
    pub fn remaining(&self) -> &'s str {
        self.s
    }
}
impl<'s> DoubleEnded<'s, StrChunks<'s>> {
    #[allow(clippy::needless_pass_by_value)]
    fn new(from: StrChunks<'s>) -> Self {
        // rust stdlib includes some optimization for Chars.count(), but its still expensive
        let char_length = from.s.chars().count();
        let rem_size = char_length % from.chunk_size;
        // SAFETY: length of valid_split_points is char_length+1, rem_size is at most char_length,
        // char_length+1 > char_length >= rem_size
        let rem_index = unsafe {
            valid_split_points(from.s)
                .nth_back(rem_size)
                .unwrap_unchecked()
                .0
        };
        let (s, remainder) = from.s.split_at(rem_index);
        Self {
            s,
            chunk_size: from.chunk_size,
            remainder,
            _inner: PhantomData,
        }
    }
}
impl<'s> DoubleEnded<'s, StrRChunks<'s>> {
    #[allow(clippy::needless_pass_by_value)]
    fn new(from: StrRChunks<'s>) -> Self {
        // rust stdlib includes some optimization for Chars.count(), but its still expensive
        let char_length = from.s.chars().count();
        let rem_size = char_length % from.chunk_size;
        // SAFETY: length of valid_split_points is char_length+1, rem_size is at most char_length,
        // char_length+1 > char_length >= rem_size
        let rem_index = unsafe {
            valid_split_points(from.s)
                .nth(rem_size)
                .unwrap_unchecked()
                .0
        };
        let (remainder, s) = from.s.split_at(rem_index);
        Self {
            s,
            chunk_size: from.chunk_size,
            remainder,
            _inner: PhantomData,
        }
    }
}
impl<'s> DoubleEnded<'s, StrChunksExact<'s>> {
    #[allow(clippy::needless_pass_by_value)]
    fn new(from: StrChunksExact<'s>) -> Self {
        let n = DoubleEnded::<'s, StrChunks<'s>>::new(StrChunks::new(from.s, from.chunk_size));
        Self {
            s: n.s,
            chunk_size: n.chunk_size,
            remainder: n.remainder,
            _inner: PhantomData,
        }
    }
    #[must_use]
    /// Returns the remainder of the original string that is not going to be returned by the
    /// iterator. the returned string has at most `chunk_size-1` characters.
    pub fn remainder(&self) -> &'s str {
        self.remainder
    }
}
impl<'s> DoubleEnded<'s, StrRChunksExact<'s>> {
    #[allow(clippy::needless_pass_by_value)]
    fn new(from: StrRChunksExact<'s>) -> Self {
        let n = DoubleEnded::<'s, StrRChunks<'s>>::new(StrRChunks::new(from.s, from.chunk_size));
        Self {
            s: n.s,
            chunk_size: n.chunk_size,
            remainder: n.remainder,
            _inner: PhantomData,
        }
    }
    #[must_use]
    /// Returns the remainder of the original string that is not going to be returned by the
    /// iterator. the returned string has at most `chunk_size-1` characters.
    pub fn remainder(&self) -> &'s str {
        self.remainder
    }
}
impl<'s> Iterator for DoubleEnded<'s, StrChunks<'s>> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        // StrChunks::new doesnt do any processing or anything, this should get optimized well
        let mut inner = StrChunks::new(self.s, self.chunk_size);
        let next = inner.next().or_else(|| {
            if self.remainder.is_empty() {
                None
            } else {
                // SAFETY: 0 is a valid code point boundary
                let (empty, remainder) =
                    unsafe { self.remainder.split_at_checked(0).unwrap_unchecked() };
                self.remainder = empty;
                Some(remainder)
            }
        });
        self.s = inner.s;
        next
    }
}
impl<'s> DoubleEndedIterator for DoubleEnded<'s, StrChunks<'s>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            let mut inner = StrRChunks::new(self.s, self.chunk_size);
            let next = inner.next();
            self.s = inner.s;
            next
        } else {
            // SAFETY: 0 is a valid code point boundary
            let (empty, remainder) =
                unsafe { self.remainder.split_at_checked(0).unwrap_unchecked() };
            self.remainder = empty;
            Some(remainder)
        }
    }
}
impl<'s> Iterator for DoubleEnded<'s, StrChunksExact<'s>> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = StrChunksExact::new(self.s, self.chunk_size);
        let next = inner.next();
        self.s = inner.s;
        next
    }
}
impl<'s> DoubleEndedIterator for DoubleEnded<'s, StrChunksExact<'s>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut inner = StrRChunks::new(self.s, self.chunk_size);
        let next = inner.next();
        self.s = inner.s;
        next
    }
}
impl<'s> Iterator for DoubleEnded<'s, StrRChunks<'s>> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = StrRChunks::new(self.s, self.chunk_size);
        let next = inner.next().or_else(|| {
            if self.remainder.is_empty() {
                None
            } else {
                // SAFETY: 0 is a valid code point boundary
                let (empty, remainder) =
                    unsafe { self.remainder.split_at_checked(0).unwrap_unchecked() };
                self.remainder = empty;
                Some(remainder)
            }
        });
        self.s = inner.s;
        next
    }
}
impl<'s> DoubleEndedIterator for DoubleEnded<'s, StrRChunks<'s>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            let mut inner = StrChunks::new(self.s, self.chunk_size);
            let next = inner.next();
            self.s = inner.s;
            next
        } else {
            // SAFETY: 0 is a valid code point boundary
            let (empty, remainder) =
                unsafe { self.remainder.split_at_checked(0).unwrap_unchecked() };
            self.remainder = empty;
            Some(remainder)
        }
    }
}
impl<'s> Iterator for DoubleEnded<'s, StrRChunksExact<'s>> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = StrRChunks::new(self.s, self.chunk_size);
        let next = inner.next();
        self.s = inner.s;
        next
    }
}
impl<'s> DoubleEndedIterator for DoubleEnded<'s, StrRChunksExact<'s>> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut inner = StrChunks::new(self.s, self.chunk_size);
        let next = inner.next();
        self.s = inner.s;
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_chunks_fore {
        ($iter:expr, $slice:expr, $remainder:expr) => {{
            let mut iter = $iter;
            let mut slice_iter = $slice.into_iter();
            loop {
                let i1 = iter.next();
                let i2 = slice_iter.next();
                if i1.is_none() && i2.is_none() {
                    // done with iteration, check remainder
                    break;
                } else {
                    assert_eq!(i1.as_ref(), i2);
                }
            }
            assert_eq!(iter.remaining(), $remainder);
            iter
        }};
    }
    macro_rules! assert_chunks_back {
        ($iter:expr, $slice:expr, $remainder:expr) => {{
            let mut iter = $iter;
            let mut slice_iter = $slice.into_iter();
            loop {
                let i1 = iter.next_back();
                let i2 = slice_iter.next_back();
                if i1.is_none() && i2.is_none() {
                    // done with iteration, check remainder
                    break;
                } else {
                    assert_eq!(i1.as_ref(), i2);
                }
            }
            assert_eq!(iter.remaining(), $remainder);
            iter
        }};
    }
    macro_rules! assert_chunks_reversable {
        ($iter:expr, $slice:expr) => {
            assert_chunks_fore!($iter.reversable(), $slice, "");
            assert_chunks_back!($iter.reversable(), $slice, "");
        };
        ($iter:expr, $slice:expr, $remainder:expr) => {
            let fore = assert_chunks_fore!($iter.reversable(), $slice, "");
            let back = assert_chunks_back!($iter.reversable(), $slice, "");
            assert_eq!(fore.remainder(), $remainder);
            assert_eq!(back.remainder(), $remainder);
        };
    }

    macro_rules! assert_chunks {
        ($iter:expr, $slice:expr, $remainder:expr) => {
            assert_chunks_fore!($iter, $slice, $remainder);
            assert_chunks_reversable!($iter, $slice);
        };
        (exact $iter:expr, $slice:expr, $remainder:expr) => {
            assert_chunks_fore!($iter, $slice, $remainder);
            assert_chunks_reversable!($iter, $slice, $remainder);
        };
    }

    #[test]
    fn str_chunks() {
        let straight_str = "012345";
        assert_chunks!(
            straight_str.str_chunks(1),
            &["0", "1", "2", "3", "4", "5"],
            ""
        );
        assert_chunks!(straight_str.str_chunks(2), &["01", "23", "45"], "");
        assert_chunks!(straight_str.str_chunks(3), &["012", "345"], "");
        assert_chunks!(straight_str.str_chunks(4), &["0123", "45"], "");
        assert_chunks!(straight_str.str_chunks(5), &["01234", "5"], "");
        assert_chunks!(straight_str.str_chunks(6), &["012345"], "");
    }
    #[test]
    fn str_chunks_exact() {
        let straight_str = "012345";
        assert_chunks!(exact
            straight_str.str_chunks_exact(1),
            &["0", "1", "2", "3", "4", "5"],
            ""
        );
        assert_chunks!(exact straight_str.str_chunks_exact(2), &["01", "23", "45"], "");
        assert_chunks!(exact straight_str.str_chunks_exact(3), &["012", "345"], "");
        assert_chunks!(exact straight_str.str_chunks_exact(4), &["0123"], "45");
        assert_chunks!(exact straight_str.str_chunks_exact(5), &["01234"], "5");
        assert_chunks!(exact straight_str.str_chunks_exact(6), &["012345"], "");
    }
    #[test]
    fn str_rchunks() {
        let straight_str = "012345";
        assert_chunks!(
            straight_str.str_rchunks(1),
            &["5", "4", "3", "2", "1", "0"],
            ""
        );
        assert_chunks!(straight_str.str_rchunks(2), &["45", "23", "01"], "");
        assert_chunks!(straight_str.str_rchunks(3), &["345", "012"], "");
        assert_chunks!(straight_str.str_rchunks(4), &["2345", "01"], "");
        assert_chunks!(straight_str.str_rchunks(5), &["12345", "0"], "");
        assert_chunks!(straight_str.str_rchunks(6), &["012345"], "");
    }
    #[test]
    fn str_rchunks_exact() {
        let straight_str = "012345";
        assert_chunks!(exact
            straight_str.str_rchunks_exact(1),
            &["5", "4", "3", "2", "1", "0"],
            ""
        );
        assert_chunks!(exact straight_str.str_rchunks_exact(2), &["45", "23", "01"], "");
        assert_chunks!(exact straight_str.str_rchunks_exact(3), &["345", "012"], "");
        assert_chunks!(exact straight_str.str_rchunks_exact(4), &["2345"], "01");
        assert_chunks!(exact straight_str.str_rchunks_exact(5), &["12345"], "0");
        assert_chunks!(exact straight_str.str_rchunks_exact(6), &["012345"], "");
    }
    #[test]
    fn emoji() {
        // these are multi-byte characters
        let s = "🥺💙😵";
        assert_chunks!(s.str_chunks(2), &["🥺💙", "😵"], "");
        assert_chunks!(exact s.str_chunks_exact(2), &["🥺💙"], "😵");
        assert_chunks!(s.str_rchunks(2), &["💙😵", "🥺"], "");
        assert_chunks!(exact s.str_rchunks_exact(2), &["💙😵"], "🥺");
    }
    #[test]
    fn empty() {
        let s = "";
        assert_chunks!(s.str_chunks(2), &[], "");
        assert_chunks!(exact s.str_chunks_exact(2), &[], "");
        assert_chunks!(s.str_rchunks(2), &[], "");
        assert_chunks!(exact s.str_rchunks_exact(2), &[], "");
    }
    #[test]
    fn short() {
        let s = "f";
        assert_chunks!(s.str_chunks(2), &["f"], "");
        assert_chunks!(exact s.str_chunks_exact(2), &[], "f");
        assert_chunks!(s.str_rchunks(2), &["f"], "");
        assert_chunks!(exact s.str_rchunks_exact(2), &[], "f");
    }
    #[test]
    fn remainder() {
        let s = "foo";
        assert_chunks!(s.str_chunks(5), &["foo"], "");
        assert_chunks!(exact s.str_chunks_exact(5), &[], "foo");
        assert_chunks!(s.str_rchunks(5), &["foo"], "");
        assert_chunks!(exact s.str_rchunks_exact(5), &[], "foo");
    }
    #[test]
    fn reverse() {
        let s = "0123456";
        assert_chunks_reversable!(s.str_chunks(3), &["012", "345", "6"]);
        assert_chunks_reversable!(s.str_rchunks(3), &["456", "123", "0"]);
        assert_chunks_reversable!(s.str_chunks_exact(3), &["012", "345"], "6");
        assert_chunks_reversable!(s.str_rchunks_exact(3), &["456", "123"], "0");
    }
}
