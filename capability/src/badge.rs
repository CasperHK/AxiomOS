//! Unforgeable process / endpoint badge.

/// An unforgeable 64-bit badge assigned to a capability at mint time.
///
/// A badge serves as an immutable identity token — e.g. which client sent a
/// message through a shared endpoint.  Because `Badge` is embedded inside
/// `Cap<T>` (which is linear / non-`Copy`), it cannot be duplicated without
/// going through `Cap::derive`, which requires appropriate rights.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Badge(u64);

impl Badge {
    /// The "no badge" / kernel-minted sentinel.
    pub const NONE: Self = Self(0);

    /// Create a badge from a raw value.
    ///
    /// Callers should ensure values are allocated monotonically to prevent
    /// reuse; see [`crate::cnode::BadgeAllocator`].
    #[inline]
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    /// Return the raw badge value.
    #[inline]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// Returns `true` if this is the sentinel "no badge".
    #[inline]
    pub const fn is_none(self) -> bool {
        self.0 == 0
    }
}

impl core::fmt::Display for Badge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Badge({})", self.0)
    }
}
