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
/// project!([unsafe] $MEMBER as $FUNCTION() -> $PROJECTION)
/// project!([unsafe] $MEMBER -> $PROJECTION)
/// project!([unsafe] $MEMBER as $FUNCTION($FROM))
/// ```
///
/// The parameters are:
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
    (unsafe $M:ident as $N:ident() -> Pin<&$T:ty>) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&Self>) -> Pin<&$T> {
            self.map_unchecked(|s| &s.$M)
        }
    };
    (unsafe $M:ident as $N:ident() -> Pin<&mut $T:ty>) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&mut Self>) -> Pin<&mut $T> {
            self.map_unchecked_mut(|s| &mut s.$M)
        }
    };
    (unsafe $M:ident as $N:ident() -> &mut $T:ty) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&mut Self>) -> &mut $T {
            &mut self.get_unchecked_mut().$M
        }
    };
    (unsafe $M:ident as $N:ident() -> &$T:ty) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    (unsafe $M:ident -> Pin<&$T:ty>) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&Self>) -> Pin<&$T> {
            self.map_unchecked(|s| &s.$M)
        }
    };
    (unsafe $M:ident -> Pin<&mut $T:ty>) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&mut Self>) -> Pin<&mut $T> {
            self.map_unchecked_mut(|s| &mut s.$M)
        }
    };
    (unsafe $M:ident -> &mut $T:ty) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&mut Self>) -> &mut $T {
            &mut self.get_unchecked_mut().$M
        }
    };
    (unsafe $M:ident as $N:ident() -> $T:ty) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };
    (unsafe $M:ident as $N:ident(&$T:ty)) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&mut Self>, from: &$T) {
            self.get_unchecked_mut().$M = from.clone();
        }
    };
    (unsafe $M:ident as $N:ident($T:ty)) => {
        #[inline]
        pub unsafe fn $N(self: Pin<&mut Self>, from: $T) {
            self.get_unchecked_mut().$M = from;
        }
    };
    (unsafe $M:ident -> &$T:ty) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    (unsafe $M:ident -> $T:ty) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };

    ($M:ident as $N:ident() -> Pin<&$T:ty>) => {
        #[inline]
        pub fn $N(self: Pin<&Self>) -> Pin<&$T> {
            unsafe { self.map_unchecked(|s| &s.$M) }
        }
    };
    ($M:ident as $N:ident() -> Pin<&mut $T:ty>) => {
        #[inline]
        pub fn $N(self: Pin<&mut Self>) -> Pin<&mut $T> {
            unsafe { self.map_unchecked_mut(|s| &mut s.$M) }
        }
    };
    ($M:ident as $N:ident() -> &mut $T:ty) => {
        #[inline]
        pub fn $N(self: Pin<&mut Self>) -> &mut $T {
            unsafe { &mut self.get_unchecked_mut().$M }
        }
    };
    ($M:ident as $N:ident() -> &$T:ty) => {
        #[inline]
        pub fn $N(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    ($M:ident as $N:ident() -> $T:ty) => {
        #[inline]
        pub fn $N(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };
    ($M:ident as $N:ident(&$T:ty)) => {
        #[inline]
        pub fn $N(self: Pin<&mut Self>, from: &$T) {
            unsafe { self.get_unchecked_mut().$M = from.clone() };
        }
    };
    ($M:ident as $N:ident($T:ty)) => {
        #[inline]
        pub fn $N(self: Pin<&mut Self>, from: $T) {
            unsafe { self.get_unchecked_mut().$M = from };
        }
    };
    ($M:ident -> Pin<&$T:ty>) => {
        #[inline]
        pub fn $M(self: Pin<&Self>) -> Pin<&$T> {
            unsafe { self.map_unchecked(|s| &s.$M) }
        }
    };
    ($M:ident -> Pin<&mut $T:ty>) => {
        #[inline]
        pub fn $M(self: Pin<&mut Self>) -> Pin<&mut $T> {
            unsafe { self.map_unchecked_mut(|s| &mut s.$M) }
        }
    };
    ($M:ident -> &mut $T:ty) => {
        #[inline]
        pub fn $M(self: Pin<&mut Self>) -> &mut $T {
            unsafe { &mut self.get_unchecked_mut().$M }
        }
    };
    ($M:ident -> &$T:ty) => {
        #[inline]
        pub fn $M(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
        }
    };
    ($M:ident -> $T:ty) => {
        #[inline]
        pub fn $M(self: Pin<&Self>) -> $T {
            self.get_ref().$M.clone()
        }
    };
}
