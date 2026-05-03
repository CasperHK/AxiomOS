//! One-shot reply capability object.

use capability::CapObject;

/// A one-shot reply object.
///
/// The kernel creates a `ReplyObj` for every `Call` IPC.  The reply capability
/// (`Cap<ReplyObj>`) is passed to the server; invoking it sends the reply and
/// immediately revokes the capability.  Because `Cap<ReplyObj>` is linear it
/// cannot be duplicated — the server can reply exactly once.
#[derive(Debug)]
pub struct ReplyObj {
    /// ID of the calling thread waiting for this reply.
    pub caller_id: u64,
    /// Whether the reply has already been sent.
    pub consumed: bool,
}

impl ReplyObj {
    /// Create a reply object for caller `caller_id`.
    #[inline]
    pub fn new(caller_id: u64) -> Self {
        Self { caller_id, consumed: false }
    }
}

impl CapObject for ReplyObj {
    fn on_revoke(&mut self) {
        // If the reply was never sent, the caller is woken with an error.
        if !self.consumed {
            // In a real kernel: wake caller with IPC error status.
        }
    }
}
