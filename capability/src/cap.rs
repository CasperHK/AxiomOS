//! `Cap<T>` — First-Class Linear-Type Capability
//!
//! The key insight of the "Ownership-is-Capability" model is that Rust's
//! ownership rules *already* enforce linearity: you cannot copy a value unless
//! it implements `Copy`, and you cannot observe it after a move.  We exploit
//! this to make capability *possession* identical to capability *authority*.
//!
//! ## Invariants upheld by the type system
//! * `Cap<T>` is **neither `Copy` nor `Clone`** — it cannot be duplicated.
//! * Moving a `Cap<T>` transfers *all* authority; the original binding is gone.
//! * Dropping a `Cap<T>` is equivalent to revoking it (calls `T::on_revoke`).
//! * Delegation (`derive`) always produces a capability with **⊆ rights**.

use crate::{Badge, CapError, Rights};

/// Trait that every kernel object managed through a capability must implement.
pub trait CapObject: Sized {
    /// Called when the last `Cap` wrapping this object is dropped.
    ///
    /// Implementations should perform deterministic cleanup (e.g. free memory,
    /// close endpoints).  The default is a no-op.
    fn on_revoke(&mut self) {}
}

/// A first-class, linear capability wrapping a kernel object of type `T`.
///
/// # Linearity
/// `Cap<T>` intentionally does **not** implement `Copy` or `Clone`.  To share
/// access you must either:
/// * **Move** the capability (full authority transfer), or
/// * **Derive** a child capability via [`Cap::derive`] (with ≤ rights).
///
/// # Example
/// ```rust
/// use capability::{Cap, Rights, Badge};
///
/// struct MyEndpoint { id: u32 }
/// impl capability::CapObject for MyEndpoint {}
///
/// let cap: Cap<MyEndpoint> = Cap::mint(MyEndpoint { id: 1 }, Rights::ALL, Badge::NONE);
/// let child = cap.derive(Rights::READ, Badge::NONE).expect("derive should succeed");
/// // `cap` was consumed by `derive`; `child` has READ-only rights.
/// drop(child); // deterministic revocation
/// ```
pub struct Cap<T: CapObject> {
    // `Option` lets us move the object out during `derive` / `into_object`
    // without requiring `unsafe` code in a type that also implements `Drop`.
    object: Option<T>,
    rights: Rights,
    badge:  Badge,
}

impl<T: CapObject> Cap<T> {
    /// Mint a brand-new capability with the given rights and badge.
    ///
    /// This is the *root* constructor — normally only the kernel calls this.
    /// User-space derives capabilities from existing ones via [`Cap::derive`].
    #[inline]
    pub fn mint(object: T, rights: Rights, badge: Badge) -> Self {
        Self { object: Some(object), rights, badge }
    }

    /// Return the rights held by this capability.
    #[inline]
    pub fn rights(&self) -> Rights {
        self.rights
    }

    /// Return the badge of this capability.
    #[inline]
    pub fn badge(&self) -> Badge {
        self.badge
    }

    /// Check whether this capability holds *at least* `required`.
    #[inline]
    pub fn has_rights(&self, required: Rights) -> bool {
        self.rights.contains(required)
    }

    /// Derive a child capability with `new_rights` and `new_badge`.
    ///
    /// # Errors
    /// * [`CapError::RightsEscalation`] — `new_rights` is not a subset of
    ///   `self.rights`.
    /// * [`CapError::InsufficientRights`] — this capability does not hold
    ///   [`Rights::GRANT`].
    ///
    /// # Note on error handling
    /// `self` is always consumed (moved into this method) regardless of
    /// success or failure.  On error the underlying object is dropped and
    /// `on_revoke` is called — the caller must not assume the parent survives
    /// a failed derivation.
    pub fn derive(mut self, new_rights: Rights, new_badge: Badge) -> Result<Cap<T>, CapError> {
        // Rights-escalation is checked first — attempting to request more than
        // the parent holds is a programming error regardless of GRANT.
        if !new_rights.is_subset_of(self.rights) {
            return Err(CapError::RightsEscalation);
        }
        if !self.rights.contains(Rights::GRANT) {
            return Err(CapError::InsufficientRights);
        }
        // Take the object out — parent is now empty (drop will not call on_revoke).
        let object = self.object.take()
            .expect("Cap internal invariant violated: object field was None before derive");        Ok(Cap {
            object: Some(object),
            rights: new_rights,
            badge:  new_badge,
        })
    }

    /// Mint a reply capability (for one-shot IPC replies).
    ///
    /// The reply cap carries only `READ | WRITE` rights and the supplied badge.
    /// It does **not** require [`Rights::GRANT`] because the kernel generates
    /// reply caps automatically on each `Call`.
    pub fn mint_reply(object: T, badge: Badge) -> Self {
        Self::mint(object, Rights::READ | Rights::WRITE, badge)
    }

    /// Unwrap the underlying object, consuming (revoking) the capability.
    ///
    /// `on_revoke` is **not** called — the caller takes full ownership of the
    /// object and is responsible for cleanup.
    #[inline]
    pub fn into_object(mut self) -> T {
        // Take the object out — parent is now empty (drop will not call on_revoke).
        self.object.take()
            .expect("Cap internal invariant violated: object field was None before into_object")
    }

    /// Immutably borrow the underlying object.
    ///
    /// Requires `required` rights.
    pub fn borrow_object(&self, required: Rights) -> Result<&T, CapError> {
        if self.rights.contains(required) {
            Ok(self.object.as_ref()
                .expect("Cap internal invariant violated: object field was None"))
        } else {
            Err(CapError::InsufficientRights)
        }
    }

    /// Mutably borrow the underlying object.
    ///
    /// Requires `required` rights.
    pub fn borrow_object_mut(&mut self, required: Rights) -> Result<&mut T, CapError> {
        if self.rights.contains(required) {
            Ok(self.object.as_mut()
                .expect("Cap internal invariant violated: object field was None"))
        } else {
            Err(CapError::InsufficientRights)
        }
    }
}

impl<T: CapObject> Drop for Cap<T> {
    /// Deterministic revocation — mirrors seL4's `cnode_delete` / `revoke`.
    ///
    /// If the object has already been moved out (via `derive` or
    /// `into_object`) this is a no-op.
    fn drop(&mut self) {
        if let Some(ref mut obj) = self.object {
            obj.on_revoke();
        }
    }
}

impl<T: CapObject + core::fmt::Debug> core::fmt::Debug for Cap<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Cap")
            .field("object", &self.object)
            .field("rights", &self.rights)
            .field("badge",  &self.badge)
            .finish()
    }
}
