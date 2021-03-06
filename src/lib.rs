#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

/// Defines a pin projection.
///
/// Projections are written inside the `impl` block of a struct.
///
/// The syntax is:
///
/// ```text
/// project!([pub] [unsafe] $MEMBER as $FUNCTION() -> $PROJECTION)
/// project!([pub] [unsafe] $MEMBER -> $PROJECTION)
/// project!([pub] [unsafe] $MEMBER as $FUNCTION($FROM))
/// ```
///
/// The parameters are:
///  - **pub** is an optional visibility specifier like `pub' or `pub(crate)`.
///  - **unsafe** is optional and generates an unsafe projection function.
///  - **MEMBER:** name of the structures member to project
///  - **FUNCTION:** name for the projection function (optional, when not given the MEMBER name is used)
///  - **PROJECTION:** resulting type
///    Type of MEMBER as:
///    - `Pin<&Type>`
///    - `Pin<&mut Type>`
///    - `&Type`
///    - `&mut Type`
///    - `Type`
///  - **FROM:** source for setters must by the type of MEMBER
///    - `Type` for a owned setter.
///    - `&Type` for a cloning setter.
///
/// The generated projection functions take `self: Pin<&Self>` or `self: Pin<&mut Self>`
/// (depending on the output type) and return the PROJECTION type or nothing for setters which
/// destroy the old value in place.
///
/// Note that almost all possible combinations (except unnamed setters) are provided. Not all
/// of the combinations make necessary sense but are provided for completeness.
///
/// # SAFETY
///
/// See [projections and structural pinning](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning)
/// at the rust standard library reference.
///
/// This library provides a thin (zero cost, fast to compile) wrapper for generating
/// projection functions. The safety thereof lies in the hands of the user!
///
#[macro_export]
macro_rules! project {
    // named, immutable, structurally pinned
    ($P:vis $M:ident as $N:ident() -> Pin<&$T:ty>) => {
        #[inline]
        $P fn $N(self: Pin<&Self>) -> Pin<&$T> {
            unsafe { self.map_unchecked(|s| &s.$M) }
        }
    };
    ($P:vis unsafe $M:ident as $N:ident() -> Pin<&$T:ty>) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&Self>) -> Pin<&$T> {
            self.map_unchecked(|s| &s.$M)
        }
    };

    // named, mutable, structurally pinned
    ($P:vis $M:ident as $N:ident() -> Pin<&mut $T:ty>) => {
        #[inline]
        $P fn $N(self: Pin<&mut Self>) -> Pin<&mut $T> {
            unsafe { self.map_unchecked_mut(|s| &mut s.$M) }
        }
    };
    ($P:vis unsafe $M:ident as $N:ident() -> Pin<&mut $T:ty>) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&mut Self>) -> Pin<&mut $T> {
            self.map_unchecked_mut(|s| &mut s.$M)
        }
    };

    // named, immutable, not structurally pinned
    ($P:vis $M:ident as $N:ident() -> &$T:ty) => {
        #[inline]
        $P fn $N(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    ($P:vis unsafe $M:ident as $N:ident() -> &$T:ty) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };

    // named, mutable, not structurally pinned
    ($P:vis $M:ident as $N:ident() -> &mut $T:ty) => {
        #[inline]
        $P fn $N(self: Pin<&mut Self>) -> &mut $T {
            unsafe { &mut self.get_unchecked_mut().$M }
        }
    };
    ($P:vis unsafe $M:ident as $N:ident() -> &mut $T:ty) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&mut Self>) -> &mut $T {
            &mut self.get_unchecked_mut().$M
        }
    };

    // unnamed, immutable, structurally pinned
    ($P:vis $M:ident -> Pin<&$T:ty>) => {
        #[inline]
        $P fn $M(self: Pin<&Self>) -> Pin<&$T> {
            unsafe { self.map_unchecked(|s| &s.$M) }
        }
    };
    ($P:vis unsafe $M:ident -> Pin<&$T:ty>) => {
        #[inline]
        $P unsafe fn $M(self: Pin<&Self>) -> Pin<&$T> {
            self.map_unchecked(|s| &s.$M)
        }
    };

    // unnamed, mutable, structurally pinned
    ($P:vis $M:ident -> Pin<&mut $T:ty>) => {
        #[inline]
        $P fn $M(self: Pin<&mut Self>) -> Pin<&mut $T> {
            unsafe { self.map_unchecked_mut(|s| &mut s.$M) }
        }
    };
    ($P:vis unsafe $M:ident -> Pin<&mut $T:ty>) => {
        #[inline]
        $P unsafe fn $M(self: Pin<&mut Self>) -> Pin<&mut $T> {
            self.map_unchecked_mut(|s| &mut s.$M)
        }
    };

    // unnamed, immutable, not structurally pinned
    ($P:vis $M:ident -> &$T:ty) => {
        #[inline]
        $P fn $M(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    ($P:vis unsafe $M:ident -> &$T:ty) => {
        #[inline]
        $P unsafe fn $M(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };

    // unnamed, mutable, not structurally pinned
    ($P:vis $M:ident -> &mut $T:ty) => {
        #[inline]
        $P fn $M(self: Pin<&mut Self>) -> &mut $T {
            unsafe { &mut self.get_unchecked_mut().$M }
        }
    };
    ($P:vis unsafe $M:ident -> &mut $T:ty) => {
        #[inline]
        $P unsafe fn $M(self: Pin<&mut Self>) -> &mut $T {
            &mut self.get_unchecked_mut().$M
        }
    };

    // named, getter, by clone
    ($P:vis $M:ident as $N:ident() -> $T:ty) => {
        #[inline]
        $P fn $N(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };
    ($P:vis unsafe $M:ident as $N:ident() -> $T:ty) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };

    // unnamed, getter, by clone
    ($P:vis $M:ident -> $T:ty) => {
        #[inline]
        $P fn $M(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };
    ($P:vis unsafe $M:ident -> $T:ty) => {
        #[inline]
        $P unsafe fn $M(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };

    // named, setter, by clone
    ($P:vis $M:ident as $N:ident(&$T:ty)) => {
        #[inline]
        $P fn $N(self: Pin<&mut Self>, from: &$T) {
            unsafe { self.get_unchecked_mut().$M = from.clone() };
        }
    };
    ($P:vis unsafe $M:ident as $N:ident(&$T:ty)) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&mut Self>, from: &$T) {
            self.get_unchecked_mut().$M = from.clone();
        }
    };

    // named, setter by move
    ($P:vis $M:ident as $N:ident($T:ty)) => {
        #[inline]
        $P fn $N(self: Pin<&mut Self>, from: $T) {
            unsafe { self.get_unchecked_mut().$M = from };
        }
    };
    ($P:vis unsafe $M:ident as $N:ident($T:ty)) => {
        #[inline]
        $P unsafe fn $N(self: Pin<&mut Self>, from: $T) {
            self.get_unchecked_mut().$M = from;
        }
    };
}
