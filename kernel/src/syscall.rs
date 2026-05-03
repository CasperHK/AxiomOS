//! Syscall dispatch layer.
//!
//! In seL4 the *only* kernel entry points are:
//! * `seL4_Call`
//! * `seL4_Send` / `seL4_NBSend`
//! * `seL4_Recv` / `seL4_NBRecv`
//! * `seL4_ReplyRecv`
//! * `seL4_Yield`
//!
//! Here we enumerate the corresponding AxiomOS syscall IDs and a stub
//! dispatcher.  Real dispatch is wired to the architecture trap handler.

/// Top-level syscall identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum Syscall {
    /// Synchronous call — send and block until reply.
    Call         = 0,
    /// Non-blocking send.
    Send         = 1,
    /// Blocking receive.
    Recv         = 2,
    /// Atomically reply and receive next message.
    ReplyRecv    = 3,
    /// Yield the current thread's timeslice.
    Yield        = 4,
    /// Retype an untyped capability.
    Retype       = 5,
    /// Copy/mint a capability into a CNode slot.
    CNodeCopy    = 6,
    /// Delete a capability from a CNode slot.
    CNodeDelete  = 7,
    /// Revoke all derived capabilities.
    CNodeRevoke  = 8,
}

impl Syscall {
    /// Decode a raw syscall number.
    pub fn from_raw(n: u64) -> Option<Self> {
        match n {
            0 => Some(Syscall::Call),
            1 => Some(Syscall::Send),
            2 => Some(Syscall::Recv),
            3 => Some(Syscall::ReplyRecv),
            4 => Some(Syscall::Yield),
            5 => Some(Syscall::Retype),
            6 => Some(Syscall::CNodeCopy),
            7 => Some(Syscall::CNodeDelete),
            8 => Some(Syscall::CNodeRevoke),
            _ => None,
        }
    }
}
