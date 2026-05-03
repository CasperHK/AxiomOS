//! CNode — capability node (table of capability slots).
//!
//! A `CNode` is the kernel structure that holds `Cap<T>` values.  In seL4 a
//! CNode is itself a kernel object accessible through a capability; here we
//! model it as a fixed-size array of `Option<CapSlot<T>>` so that all slot
//! operations are checked at compile-time and at runtime without unsafe code.
//!
//! `BadgeAllocator` provides monotonically-increasing badge IDs to prevent
//! reuse across the lifetime of a capability.

use crate::{Badge, Cap, CapError, CapObject, Rights};

/// A single slot in a [`CNode`].
///
/// The slot wraps an `Option<Cap<T>>` and provides checked insert / remove.
pub struct CapSlot<T: CapObject> {
    cap: Option<Cap<T>>,
}

impl<T: CapObject> CapSlot<T> {
    /// Create an empty slot.
    #[inline]
    pub const fn empty() -> Self {
        Self { cap: None }
    }

    /// Returns `true` when the slot holds a capability.
    #[inline]
    pub fn is_occupied(&self) -> bool {
        self.cap.is_some()
    }

    /// Insert `cap` into this slot.
    ///
    /// # Errors
    /// [`CapError::SlotOccupied`] if the slot already holds a capability.
    pub fn insert(&mut self, cap: Cap<T>) -> Result<(), CapError> {
        if self.cap.is_some() {
            return Err(CapError::SlotOccupied);
        }
        self.cap = Some(cap);
        Ok(())
    }

    /// Remove and return the capability from this slot.
    ///
    /// # Errors
    /// [`CapError::SlotEmpty`] if the slot is empty.
    pub fn remove(&mut self) -> Result<Cap<T>, CapError> {
        self.cap.take().ok_or(CapError::SlotEmpty)
    }

    /// Borrow the capability immutably.
    ///
    /// # Errors
    /// [`CapError::SlotEmpty`] if the slot is empty.
    pub fn borrow(&self) -> Result<&Cap<T>, CapError> {
        self.cap.as_ref().ok_or(CapError::SlotEmpty)
    }

    /// Borrow the capability mutably.
    ///
    /// # Errors
    /// [`CapError::SlotEmpty`] if the slot is empty.
    pub fn borrow_mut(&mut self) -> Result<&mut Cap<T>, CapError> {
        self.cap.as_mut().ok_or(CapError::SlotEmpty)
    }
}

impl<T: CapObject> Drop for CapSlot<T> {
    /// Dropping a slot revokes any capability it holds.
    fn drop(&mut self) {
        // `self.cap` will be dropped here, triggering `Cap::drop` →
        // `T::on_revoke`.
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// CNode — fixed-size capability table
// ──────────────────────────────────────────────────────────────────────────────

/// A capability node: a fixed-size table of `N` capability slots.
///
/// # Type Parameters
/// * `T` — the kernel object type stored in every slot of this CNode.
/// * `N` — the compile-time number of slots (must be > 0).
///
/// # Relationship to seL4
/// In seL4 a CNode is a kernel object that stores capability slots.  An address
/// space root CNode is itself referenced via a capability.  Here we make the
/// CNode a plain Rust generic struct; the kernel holds its root CNode via a
/// `Cap<CNode<…>>` for the same effect.
pub struct CNode<T: CapObject, const N: usize> {
    slots: [CapSlot<T>; N],
}

impl<T: CapObject, const N: usize> CNode<T, N> {
    /// Create a CNode with all `N` slots empty.
    pub fn new() -> Self {
        // `CapSlot::empty()` is `const fn` so we can initialise the array.
        Self {
            slots: core::array::from_fn(|_| CapSlot::empty()),
        }
    }

    /// Number of slots in this CNode.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Insert `cap` into slot `index`.
    ///
    /// # Errors
    /// * [`CapError::SlotOutOfRange`] — `index >= N`.
    /// * [`CapError::SlotOccupied`]   — slot already holds a capability.
    pub fn insert(&mut self, index: usize, cap: Cap<T>) -> Result<(), CapError> {
        let slot = self.slot_mut(index)?;
        slot.insert(cap)
    }

    /// Remove and return the capability at slot `index`.
    ///
    /// # Errors
    /// * [`CapError::SlotOutOfRange`] — `index >= N`.
    /// * [`CapError::SlotEmpty`]      — slot is empty.
    pub fn remove(&mut self, index: usize) -> Result<Cap<T>, CapError> {
        let slot = self.slot_mut(index)?;
        slot.remove()
    }

    /// Borrow the capability at slot `index` immutably.
    ///
    /// # Errors
    /// * [`CapError::SlotOutOfRange`] — `index >= N`.
    /// * [`CapError::SlotEmpty`]      — slot is empty.
    pub fn borrow(&self, index: usize) -> Result<&Cap<T>, CapError> {
        let slot = self.slots.get(index).ok_or(CapError::SlotOutOfRange)?;
        slot.borrow()
    }

    /// Borrow the capability at slot `index` mutably.
    ///
    /// # Errors
    /// * [`CapError::SlotOutOfRange`] — `index >= N`.
    /// * [`CapError::SlotEmpty`]      — slot is empty.
    pub fn borrow_mut(&mut self, index: usize) -> Result<&mut Cap<T>, CapError> {
        let slot = self.slot_mut(index)?;
        slot.borrow_mut()
    }

    /// Return the number of occupied slots.
    pub fn occupied_count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_occupied()).count()
    }

    // ── helpers ──────────────────────────────────────────────────────────────

    fn slot_mut(&mut self, index: usize) -> Result<&mut CapSlot<T>, CapError> {
        self.slots.get_mut(index).ok_or(CapError::SlotOutOfRange)
    }
}

impl<T: CapObject, const N: usize> Default for CNode<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Badge allocator
// ──────────────────────────────────────────────────────────────────────────────

/// A monotonically-increasing badge allocator.
///
/// Guarantees that each allocated badge is unique within the lifetime of a
/// running kernel instance.  Badge `0` is reserved as [`Badge::NONE`].
pub struct BadgeAllocator {
    next: u64,
}

impl BadgeAllocator {
    /// Create a new allocator.  The first allocated badge will be `1`.
    pub const fn new() -> Self {
        Self { next: 1 }
    }

    /// Allocate the next unique badge.
    ///
    /// # Panics
    /// Panics (in debug) / wraps (in release) when all 2^64-1 badges are
    /// exhausted — effectively impossible in practice.
    pub fn alloc(&mut self) -> Badge {
        let b = Badge::new(self.next);
        self.next = self.next.wrapping_add(1);
        b
    }

    /// Derive a child capability from `parent` with `new_rights` and a fresh
    /// unique badge.
    ///
    /// # Errors
    /// Propagates errors from [`Cap::derive`].
    pub fn derive_with_fresh_badge<T: CapObject>(
        &mut self,
        parent: Cap<T>,
        new_rights: Rights,
    ) -> Result<Cap<T>, CapError> {
        let badge = self.alloc();
        parent.derive(new_rights, badge)
    }
}

impl Default for BadgeAllocator {
    fn default() -> Self {
        Self::new()
    }
}
