//! IPC Endpoint — a rendezvous point for synchronous message passing.

use capability::CapObject;
use crate::message::Message;

/// The state of an IPC endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointState {
    /// No thread is currently waiting.
    Idle,
    /// One or more threads are blocked waiting to *send*.
    SendBlocked,
    /// One or more threads are blocked waiting to *receive*.
    RecvBlocked,
}

/// An IPC endpoint kernel object.
///
/// Endpoints are the *sole* inter-process communication primitive.  All higher-
/// level abstractions (shared memory, signals, distributed bus) are layered on
/// top of or alongside endpoints.
///
/// In the "Ownership-is-Capability" model an endpoint is always accessed
/// through `Cap<Endpoint>`.  A `Cap` with `Rights::WRITE` may send; one with
/// `Rights::READ` may receive.
#[derive(Debug)]
pub struct Endpoint {
    /// Unique kernel-assigned ID.
    pub id: u64,
    /// Current rendezvous state.
    pub state: EndpointState,
    /// Buffered message (set when a sender blocks before a receiver arrives).
    pub pending_message: Option<Message>,
}

impl Endpoint {
    /// Create a new idle endpoint.
    #[inline]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: EndpointState::Idle,
            pending_message: None,
        }
    }

    /// Simulate sending a message to this endpoint.
    ///
    /// In a real kernel this would block the caller if no receiver is ready.
    /// Here we model it as storing the message.
    pub fn send(&mut self, msg: Message) {
        self.pending_message = Some(msg);
        self.state = EndpointState::SendBlocked;
    }

    /// Simulate receiving a message from this endpoint.
    ///
    /// Returns `Some(Message)` if a sender is waiting, `None` otherwise.
    pub fn recv(&mut self) -> Option<Message> {
        if let Some(msg) = self.pending_message.take() {
            self.state = EndpointState::Idle;
            Some(msg)
        } else {
            self.state = EndpointState::RecvBlocked;
            None
        }
    }
}

impl CapObject for Endpoint {
    fn on_revoke(&mut self) {
        // Wake any blocked sender/receiver with an error.
        self.state = EndpointState::Idle;
        self.pending_message = None;
    }
}
