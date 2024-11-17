use cmp::Ordering;

use {
    crate::TextSize,
    std::{
        cmp, fmt,
        ops::{Add, AddAssign, Bound, Index, IndexMut, Range, RangeBounds, Sub, SubAssign},
    },
};

/// A span in text, represented as a pair of [`TextSize`][struct@TextSize].
///
/// It is a logic error for `start` to be greater than `end`.
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span {
    // Invariant: start <= end
    start: TextSize,
    end: TextSize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start().raw, self.end().raw)
    }
}

impl Span {
    /// Creates a new `Span` with the given `start` and `end` (`start..end`).
    ///
    /// # Panics
    ///
    /// Panics if `end < start`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let start = TextSize::from(5);
    /// let end = TextSize::from(10);
    /// let span = Span::new(start, end);
    ///
    /// assert_eq!(span.start(), start);
    /// assert_eq!(span.end(), end);
    /// assert_eq!(span.len(), end - start);
    /// ```
    #[inline]
    pub const fn new(start: TextSize, end: TextSize) -> Span {
        assert!(start.raw <= end.raw);
        Span { start, end }
    }

    /// Create a new `Span` with the given `offset` and `len` (`offset..offset + len`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let text = "0123456789";
    ///
    /// let offset = TextSize::from(2);
    /// let length = TextSize::from(5);
    /// let span = Span::at(offset, length);
    ///
    /// assert_eq!(span, Span::new(offset, offset + length));
    /// assert_eq!(&text[span], "23456")
    /// ```
    #[inline]
    pub const fn at(offset: TextSize, len: TextSize) -> Span {
        Span::new(offset, TextSize::new(offset.raw + len.raw))
    }

    /// Create a zero-length span at the specified offset (`offset..offset`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let point: TextSize;
    /// # point = TextSize::from(3);
    /// let span = Span::empty(point);
    /// assert!(span.is_empty());
    /// assert_eq!(span, Span::new(point, point));
    /// ```
    #[inline]
    pub const fn empty(offset: TextSize) -> Span {
        Span {
            start: offset,
            end: offset,
        }
    }

    /// Create a span up to the given end (`..end`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let point: TextSize;
    /// # point = TextSize::from(12);
    /// let span = Span::up_to(point);
    ///
    /// assert_eq!(span.len(), point);
    /// assert_eq!(span, Span::new(0.into(), point));
    /// assert_eq!(span, Span::at(0.into(), point));
    /// ```
    #[inline]
    pub const fn up_to(end: TextSize) -> Span {
        Span {
            start: TextSize::new(0),
            end,
        }
    }
}

/// Identity methods.
impl Span {
    /// The start point of this span.
    #[inline]
    pub const fn start(self) -> TextSize {
        self.start
    }

    /// The end point of this span.
    #[inline]
    pub const fn end(self) -> TextSize {
        self.end
    }

    /// The size of this span.
    #[inline]
    pub const fn len(self) -> TextSize {
        // HACK for const fn: math on primitives only
        TextSize {
            raw: self.end().raw - self.start().raw,
        }
    }

    /// Check if this span is empty.
    #[inline]
    pub const fn is_empty(self) -> bool {
        // HACK for const fn: math on primitives only
        self.start().raw == self.end().raw
    }
}

/// Manipulation methods.
impl Span {
    /// Check if this span contains an offset.
    ///
    /// The end index is considered excluded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let (start, end): (TextSize, TextSize);
    /// # start = 10.into(); end = 20.into();
    /// let span = Span::new(start, end);
    /// assert!(span.contains(start));
    /// assert!(!span.contains(end));
    /// ```
    #[inline]
    pub fn contains(self, offset: TextSize) -> bool {
        self.start() <= offset && offset < self.end()
    }

    /// Check if this span contains an offset.
    ///
    /// The end index is considered included.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let (start, end): (TextSize, TextSize);
    /// # start = 10.into(); end = 20.into();
    /// let span = Span::new(start, end);
    /// assert!(span.contains_inclusive(start));
    /// assert!(span.contains_inclusive(end));
    /// ```
    #[inline]
    pub fn contains_inclusive(self, offset: TextSize) -> bool {
        self.start() <= offset && offset <= self.end()
    }

    /// Check if this span completely contains another span.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// let larger = Span::new(0.into(), 20.into());
    /// let smaller = Span::new(5.into(), 15.into());
    /// assert!(larger.contains_span(smaller));
    /// assert!(!smaller.contains_span(larger));
    ///
    /// // a span always contains itself
    /// assert!(larger.contains_span(larger));
    /// assert!(smaller.contains_span(smaller));
    /// ```
    #[inline]
    pub fn contains_span(self, other: Span) -> bool {
        self.start() <= other.start() && other.end() <= self.end()
    }

    /// The span covered by both ranges, if it exists.
    /// If the ranges touch but do not overlap, the output span is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// assert_eq!(
    ///     Span::intersect(
    ///         Span::new(0.into(), 10.into()),
    ///         Span::new(5.into(), 15.into()),
    ///     ),
    ///     Some(Span::new(5.into(), 10.into())),
    /// );
    /// ```
    #[inline]
    pub fn intersect(self, other: Span) -> Option<Span> {
        let start = cmp::max(self.start(), other.start());
        let end = cmp::min(self.end(), other.end());
        if end < start {
            return None;
        }
        Some(Span::new(start, end))
    }

    /// Extends the span to cover `other` as well.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// assert_eq!(
    ///     Span::cover(
    ///         Span::new(0.into(), 5.into()),
    ///         Span::new(15.into(), 20.into()),
    ///     ),
    ///     Span::new(0.into(), 20.into()),
    /// );
    /// ```
    #[inline]
    pub fn cover(self, other: Span) -> Span {
        let start = cmp::min(self.start(), other.start());
        let end = cmp::max(self.end(), other.end());
        Span::new(start, end)
    }

    /// Extends the span to cover `other` offsets as well.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use text::*;
    /// assert_eq!(
    ///     Span::empty(0.into()).cover_offset(20.into()),
    ///     Span::new(0.into(), 20.into()),
    /// )
    /// ```
    #[inline]
    pub fn cover_offset(self, offset: TextSize) -> Span {
        self.cover(Span::empty(offset))
    }

    /// Add an offset to this span.
    ///
    /// Note that this is not appropriate for changing where a `Span` is
    /// within some string; rather, it is for changing the reference anchor
    /// that the `Span` is measured against.
    ///
    /// The unchecked version (`Add::add`) will _always_ panic on overflow,
    /// in contrast to primitive integers, which check in debug mode only.
    #[inline]
    pub fn checked_add(self, offset: TextSize) -> Option<Span> {
        Some(Span {
            start: self.start.checked_add(offset)?,
            end: self.end.checked_add(offset)?,
        })
    }

    /// Subtract an offset from this span.
    ///
    /// Note that this is not appropriate for changing where a `Span` is
    /// within some string; rather, it is for changing the reference anchor
    /// that the `Span` is measured against.
    ///
    /// The unchecked version (`Sub::sub`) will _always_ panic on overflow,
    /// in contrast to primitive integers, which check in debug mode only.
    #[inline]
    pub fn checked_sub(self, offset: TextSize) -> Option<Span> {
        Some(Span {
            start: self.start.checked_sub(offset)?,
            end: self.end.checked_sub(offset)?,
        })
    }

    /// Relative order of the two ranges (overlapping ranges are considered
    /// equal).
    ///
    ///
    /// This is useful when, for example, binary searching an array of disjoint
    /// ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// # use text::*;
    /// # use std::cmp::Ordering;
    ///
    /// let a = Span::new(0.into(), 3.into());
    /// let b = Span::new(4.into(), 5.into());
    /// assert_eq!(a.ordering(b), Ordering::Less);
    ///
    /// let a = Span::new(0.into(), 3.into());
    /// let b = Span::new(3.into(), 5.into());
    /// assert_eq!(a.ordering(b), Ordering::Less);
    ///
    /// let a = Span::new(0.into(), 3.into());
    /// let b = Span::new(2.into(), 5.into());
    /// assert_eq!(a.ordering(b), Ordering::Equal);
    ///
    /// let a = Span::new(0.into(), 3.into());
    /// let b = Span::new(2.into(), 2.into());
    /// assert_eq!(a.ordering(b), Ordering::Equal);
    ///
    /// let a = Span::new(2.into(), 3.into());
    /// let b = Span::new(2.into(), 2.into());
    /// assert_eq!(a.ordering(b), Ordering::Greater);
    /// ```
    #[inline]
    pub fn ordering(self, other: Span) -> Ordering {
        if self.end() <= other.start() {
            Ordering::Less
        } else if other.end() <= self.start() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Index<Span> for str {
    type Output = str;
    #[inline]
    fn index(&self, index: Span) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl Index<Span> for String {
    type Output = str;
    #[inline]
    fn index(&self, index: Span) -> &str {
        &self[Range::<usize>::from(index)]
    }
}

impl IndexMut<Span> for str {
    #[inline]
    fn index_mut(&mut self, index: Span) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}

impl IndexMut<Span> for String {
    #[inline]
    fn index_mut(&mut self, index: Span) -> &mut str {
        &mut self[Range::<usize>::from(index)]
    }
}

impl RangeBounds<TextSize> for Span {
    fn start_bound(&self) -> Bound<&TextSize> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&TextSize> {
        Bound::Excluded(&self.end)
    }
}

impl From<(crate::Offset, crate::Offset)> for Span {
    #[inline]
    fn from((start, end): (crate::Offset, crate::Offset)) -> Self {
        Span::new(start.into(), end.into())
    }
}

impl<T> From<Span> for Range<T>
where
    T: From<TextSize>,
{
    #[inline]
    fn from(r: Span) -> Self {
        r.start().into()..r.end().into()
    }
}

macro_rules! ops {
    (impl $Op:ident for Span by fn $f:ident = $op:tt) => {
        impl $Op<&TextSize> for Span {
            type Output = Span;
            #[inline]
            fn $f(self, other: &TextSize) -> Span {
                self $op *other
            }
        }
        impl<T> $Op<T> for &Span
        where
            Span: $Op<T, Output=Span>,
        {
            type Output = Span;
            #[inline]
            fn $f(self, other: T) -> Span {
                *self $op other
            }
        }
    };
}

impl Add<TextSize> for Span {
    type Output = Span;
    #[inline]
    fn add(self, offset: TextSize) -> Span {
        self.checked_add(offset).expect("Span +offset overflowed")
    }
}

impl Sub<TextSize> for Span {
    type Output = Span;
    #[inline]
    fn sub(self, offset: TextSize) -> Span {
        self.checked_sub(offset).expect("Span -offset overflowed")
    }
}

ops!(impl Add for Span by fn add = +);
ops!(impl Sub for Span by fn sub = -);

impl<A> AddAssign<A> for Span
where
    Span: Add<A, Output = Span>,
{
    #[inline]
    fn add_assign(&mut self, rhs: A) {
        *self = *self + rhs
    }
}

impl<S> SubAssign<S> for Span
where
    Span: Sub<S, Output = Span>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: S) {
        *self = *self - rhs
    }
}
