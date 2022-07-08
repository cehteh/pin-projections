#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

/// Defines a pin projection.
///
/// SAFETY: see [projections and structural pinning](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning)
/// at the rust standard library reference.
///
/// Projections are written inside the `impl` block of a struct.
///
/// The syntax is:
///
/// ```text
/// project!($MEMBER as $FUNCTION() -> $PROJECTION)
/// project!(unsafe $MEMBER as $FUNCTION() -> $PROJECTION)
/// project!($MEMBER -> $PROJECTION)
/// project!(unsafe $MEMBER -> $PROJECTION)
/// ```
///
/// The parameters are:
///  - **MEMBER:** name of the structures member to project
///  - **FUNCTION:** name for the projection function (optional, when not given the MEMBER name is used)
///  - **PROJECTION:** resulting type (Type of MEMBER as `Pin<&Type>`, `Pin<&mut Type>`, `&Type` or `&mut Type`
///  - When **unsafe** is used a unsafe projection function is generated.
///
/// The generated projection functions take `self: Pin<&Self>` or `self: Pin<&mut Self>` and return the
/// PROJECTION type.
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
    (unsafe $M:ident -> &$T:ty) => {
        #[inline]
        pub unsafe fn $M(self: Pin<&Self>) -> &$T {
            &self.get_ref().$M
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
}
