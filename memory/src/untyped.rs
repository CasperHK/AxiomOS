//! Untyped memory — raw physical memory before retyping.
//!
//! In seL4 *all* physical memory begins as `UntypedMemory`.  The only way to
//! create a kernel object (Frame, Endpoint, CNode, …) is to `retype` a slice
//! of untyped memory.  Possession of `Cap<UntypedMemory>` is the authority to
//! perform that retyping — guaranteeing that the same physical memory cannot be
//! doubly-allocated.

use capability::CapObject;

/// Raw physical memory awaiting retyping.
#[derive(Debug)]
pub struct UntypedMemory {
    /// Physical base address.
    pub phys_addr: u64,
    /// Size in bytes (must be a power of two per seL4 convention).
    pub size_bits: u8,
    /// Whether this region has already been retyped (consumed).
    pub retyped: bool,
}

impl UntypedMemory {
    /// Create an untyped region.
    ///
    /// # Parameters
    /// * `phys_addr` — physical base address (must be `2^size_bits`-aligned).
    /// * `size_bits` — log₂ of the region size.
    pub fn new(phys_addr: u64, size_bits: u8) -> Self {
        Self { phys_addr, size_bits, retyped: false }
    }

    /// Return the size of this region in bytes.
    pub fn size_bytes(&self) -> u64 {
        1u64 << self.size_bits
    }

    /// Mark this region as consumed (retyped).
    ///
    /// Once retyped, the memory is owned by the derived object's capability
    /// and this `UntypedMemory` object becomes invalid.
    pub fn consume(&mut self) {
        self.retyped = true;
    }

    /// Returns `true` if the region can still be retyped.
    pub fn is_available(&self) -> bool {
        !self.retyped
    }
}

impl CapObject for UntypedMemory {
    fn on_revoke(&mut self) {
        // Returning untyped memory to the free pool would happen here.
    }
}
