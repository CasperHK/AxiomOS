//! Physical memory frame object.

use capability::CapObject;

/// Supported page / frame sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSize {
    /// 4 KiB small page (all architectures).
    Small,
    /// 2 MiB large page (x86-64 / AArch64).
    Large,
    /// 1 GiB huge page (x86-64).
    Huge,
}

impl FrameSize {
    /// Size in bytes.
    pub const fn bytes(self) -> usize {
        match self {
            FrameSize::Small => 4 * 1024,
            FrameSize::Large => 2 * 1024 * 1024,
            FrameSize::Huge  => 1024 * 1024 * 1024,
        }
    }
}

/// A physical memory frame (page).
///
/// `Cap<Frame>` represents the authority to map this frame into an address
/// space.  Without the capability the frame is inaccessible.
#[derive(Debug)]
pub struct Frame {
    /// Physical base address of this frame.
    pub phys_addr: u64,
    /// Size of this frame.
    pub size: FrameSize,
    /// Whether this frame is currently mapped into any address space.
    pub mapped: bool,
}

impl Frame {
    /// Create a frame at the given physical address.
    #[inline]
    pub fn new(phys_addr: u64, size: FrameSize) -> Self {
        Self { phys_addr, size, mapped: false }
    }

    /// Mark the frame as mapped.
    ///
    /// Requires `Rights::WRITE` on the wrapping `Cap<Frame>`.
    pub fn map(&mut self) {
        self.mapped = true;
    }

    /// Unmap the frame.
    pub fn unmap(&mut self) {
        self.mapped = false;
    }
}

impl CapObject for Frame {
    fn on_revoke(&mut self) {
        // Automatically unmap when the last cap is revoked.
        self.unmap();
    }
}
