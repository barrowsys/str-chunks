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
//! the methods `str_chunks`, `str_chunks_exact`, `str_rchunks`, and `str_rchunks_exact`
//! behave like the similarly named methods on slice,
//! but return string slices that are `chunk_size` chars long.
//! take note: these slices are not necessarily `chunk_size` *bytes* long. `chunk.len() != chunk_size`
//!
//! import `ImplStrChunks` to get methods on `str`

use core::num::NonZeroUsize;

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
    fn str_chunks(&self, chunk_size: usize) -> StrChunks;
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the beginning of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last up to `chunk_size-1` chars will be omitted and can be retrieved from the
    /// [remaining](`StrChunksExact::remaining`) function of the iterator after the iterator has returned None.
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
    fn str_rchunks(&self, chunk_size: usize) -> StrRChunks;
    /// Returns an iterator over `chunk_size` chars at a time,
    /// starting at the end of the str.
    /// The chunks are &str slices and do not overlap.
    ///
    /// If `chunk_size` does not divide the char-length of the str,
    /// then the last up to `chunk_size-1` chars will be omitted and can be retrieved from the
    /// [remaining](`StrRChunksExact::remaining`) function of the iterator after the iterator has
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
fn valid_split_points(
    s: &str,
) -> core::iter::Chain<core::str::CharIndices, core::iter::Once<(usize, char)>> {
    s.char_indices()
        .chain(core::iter::once((s.len(), char::REPLACEMENT_CHARACTER)))
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
}

impl<'s> Iterator for StrChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let (index, _) = valid_split_points(self.s)
            .nth(self.chunk_size.get())
            // SAFETY: the end of a str is a valid byte offset, and is the boundary of a codepoint
            // by definition of a str.
            .unwrap_or((self.s.len(), char::REPLACEMENT_CHARACTER));
        // SAFETY: index is a valid byte offset on the boundary of a code point by above
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
}

impl<'s> Iterator for StrChunksExact<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        // will short-circuit None if s.len() < chunk_size, leaving the remainder in s
        let (index, _) = valid_split_points(self.s).nth(self.chunk_size.get())?;

        // SAFETY: index is a valid byte offset on the boundary of a code point by above
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
}

impl<'s> Iterator for StrRChunks<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        let (index, _) = valid_split_points(self.s)
            .nth_back(self.chunk_size.get())
            // SAFETY: the start of a str is a valid byte offset, and is the boundary of a codepoint
            // by definition of a str.
            .unwrap_or((0, char::REPLACEMENT_CHARACTER));
        // SAFETY: index is a valid byte offset on the boundary of a code point by above
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
/// `chunk_size-1` chars can will be omitted but can be retrieved with the [remaining] function
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
}

impl<'s> Iterator for StrRChunksExact<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<&'s str> {
        // will short-circuit None if s.len() < chunk_size, leaving the remainder in s
        let (index, _) = valid_split_points(self.s).nth_back(self.chunk_size.get())?;

        // SAFETY: index is a valid byte offset on the boundary of a code point by above
        let (rest, chunk) = unsafe { self.s.split_at_checked(index).unwrap_unchecked() };

        self.s = rest;
        Some(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // need this to be no_std
    macro_rules! assert_chunks {
        ($iter:expr, $slice:expr, $remainder:expr) => {
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
        assert_chunks!(
            straight_str.str_chunks_exact(1),
            &["0", "1", "2", "3", "4", "5"],
            ""
        );
        assert_chunks!(straight_str.str_chunks_exact(2), &["01", "23", "45"], "");
        assert_chunks!(straight_str.str_chunks_exact(3), &["012", "345"], "");
        assert_chunks!(straight_str.str_chunks_exact(4), &["0123"], "45");
        assert_chunks!(straight_str.str_chunks_exact(5), &["01234"], "5");
        assert_chunks!(straight_str.str_chunks_exact(6), &["012345"], "");
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
        assert_chunks!(
            straight_str.str_rchunks_exact(1),
            &["5", "4", "3", "2", "1", "0"],
            ""
        );
        assert_chunks!(straight_str.str_rchunks_exact(2), &["45", "23", "01"], "");
        assert_chunks!(straight_str.str_rchunks_exact(3), &["345", "012"], "");
        assert_chunks!(straight_str.str_rchunks_exact(4), &["2345"], "01");
        assert_chunks!(straight_str.str_rchunks_exact(5), &["12345"], "0");
        assert_chunks!(straight_str.str_rchunks_exact(6), &["012345"], "");
    }
    #[test]
    fn emoji() {
        // these are multi-byte characters
        let s = "🥺💙😵";
        assert_chunks!(s.str_chunks(2), &["🥺💙", "😵"], "");
        assert_chunks!(s.str_chunks_exact(2), &["🥺💙"], "😵");
        assert_chunks!(s.str_rchunks(2), &["💙😵", "🥺"], "");
        assert_chunks!(s.str_rchunks_exact(2), &["💙😵"], "🥺");
    }
    #[test]
    fn empty() {
        let s = "";
        assert_chunks!(s.str_chunks(2), &[], "");
        assert_chunks!(s.str_chunks_exact(2), &[], "");
        assert_chunks!(s.str_rchunks(2), &[], "");
        assert_chunks!(s.str_rchunks_exact(2), &[], "");
    }
    #[test]
    fn short() {
        let s = "f";
        assert_chunks!(s.str_chunks(2), &["f"], "");
        assert_chunks!(s.str_chunks_exact(2), &[], "f");
        assert_chunks!(s.str_rchunks(2), &["f"], "");
        assert_chunks!(s.str_rchunks_exact(2), &[], "f");
    }
}
