//! Capability rights — a bitflags set that controls what a holder may do.

use bitflags::bitflags;

bitflags! {
    /// The rights associated with a capability.
    ///
    /// Rights are stored inside [`crate::Cap`] and are always a *subset* of the
    /// minting capability's rights.  Attempting to derive a capability with
    /// *more* rights than the parent will fail with
    /// [`crate::CapError::InsufficientRights`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Rights: u32 {
        /// Holder may read from / receive on the underlying object.
        const READ    = 0b0000_0001;
        /// Holder may write to / send on the underlying object.
        const WRITE   = 0b0000_0010;
        /// Holder may grant (delegate) this capability to others.
        const GRANT   = 0b0000_0100;
        /// Holder may grant a *copy* (mint) to others.
        const GRANT_REPLY = 0b0000_1000;
        /// Holder may revoke derived capabilities.
        const REVOKE  = 0b0001_0000;
        /// Full rights shorthand.
        const ALL     = Self::READ.bits()
                      | Self::WRITE.bits()
                      | Self::GRANT.bits()
                      | Self::GRANT_REPLY.bits()
                      | Self::REVOKE.bits();
    }
}

impl Rights {
    /// Returns `true` when `self` is a (non-strict) subset of `parent`.
    ///
    /// Used to enforce the *rights-subset* invariant during capability
    /// derivation.
    #[inline]
    pub fn is_subset_of(self, parent: Rights) -> bool {
        (self & parent) == self
    }
}

impl Default for Rights {
    fn default() -> Self {
        Rights::ALL
    }
}
