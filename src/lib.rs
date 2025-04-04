//! Traits for conversions between non-local types.
//!
//! The traits in this crate provide a way to convert from one type to another type.
//! Each trait serves a different purpose:
//!
//! - Implement the [`From`] trait for consuming value-to-value conversions
//! - Implement the [`Into`] trait for consuming value-to-value conversions to types
//!   outside the current crate
//! - The [`TryFrom`] and [`TryInto`] traits behave like [`From`] and [`Into`],
//!   but should be implemented when the conversion can fail.
#![no_std]

/// A value-to-value conversion that consumes the input value. The
/// opposite of [`Fromage`].
///
/// See [`From`] for documentation on the trait.
pub trait Intoage<T, X>: Sized {
    /// Converts this type into the (usually inferred) input type.
    #[must_use]
    fn intoage(self) -> T;
}

/// Used to do value-to-value conversions while consuming the input value. It is the reciprocal of
/// [`Intoage`].
///
/// See [`Into`] for documentation on the trait.
pub trait Fromage<T, X>: Sized {
    /// Converts to this type from the input type.
    #[must_use]
    fn fromage(value: T) -> Self;
}

/// An attempted conversion that consumes `self`, which may or may not be
/// expensive.
///
/// See [`TryInto`] for documentation on the trait.
pub trait TryIntoage<T, X>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_intoage(self) -> Result<T, Self::Error>;
}

/// Simple and safe type conversions that may fail in a controlled
/// way under some circumstances. It is the reciprocal of [`TryIntoage`].
///
/// See [`TryFrom`] for documentation on the trait.
pub trait TryFromage<T, X>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn try_fromage(value: T) -> Result<Self, Self::Error>;
}

// From implies Into
impl<T, U, X> Intoage<U, X> for T
where
    U: Fromage<T, X>,
{
    /// Calls `U::fromage(self)`.
    ///
    /// That is, this conversion is whatever the implementation of
    /// <code>[Fromage]&lt;T&gt; for U</code> chooses to do.
    #[inline]
    #[track_caller]
    fn intoage(self) -> U {
        U::fromage(self)
    }
}

// From (and thus Into) is reflexive
impl<T> Fromage<T, ()> for T {
    /// Returns the argument unchanged.
    #[inline(always)]
    fn fromage(t: T) -> T {
        t
    }
}

// TryFrom implies TryInto
impl<T, U, X> TryIntoage<U, X> for T
where
    U: TryFromage<T, X>,
{
    type Error = U::Error;

    #[inline]
    fn try_intoage(self) -> Result<U, U::Error> {
        U::try_fromage(self)
    }
}

// Infallible conversions are semantically equivalent to fallible conversions
// with an uninhabited error type.
impl<T, U> TryFromage<U, ()> for T
where
    U: Intoage<T, ()>,
{
    type Error = core::convert::Infallible;

    #[inline]
    fn try_fromage(value: U) -> Result<Self, Self::Error> {
        Ok(U::intoage(value))
    }
}
