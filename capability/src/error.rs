//! Capability errors.

/// All errors that capability operations can produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapError {
    /// The requested operation requires rights not present in this capability.
    InsufficientRights,
    /// The CNode slot targeted is already occupied.
    SlotOccupied,
    /// The CNode slot targeted is out of range.
    SlotOutOfRange,
    /// The CNode is empty (slot is `None`) where a capability was expected.
    SlotEmpty,
    /// A badge value of zero was supplied where a non-zero badge is required.
    InvalidBadge,
    /// Attempted to derive more rights than the parent possesses.
    RightsEscalation,
}

impl core::fmt::Display for CapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CapError::InsufficientRights  => write!(f, "capability: insufficient rights"),
            CapError::SlotOccupied        => write!(f, "capability: cnode slot already occupied"),
            CapError::SlotOutOfRange      => write!(f, "capability: cnode slot index out of range"),
            CapError::SlotEmpty           => write!(f, "capability: cnode slot is empty"),
            CapError::InvalidBadge        => write!(f, "capability: invalid badge value (zero)"),
            CapError::RightsEscalation    => write!(f, "capability: rights escalation attempt"),
        }
    }
}
