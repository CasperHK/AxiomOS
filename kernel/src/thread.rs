//! Kernel thread / TCB (Thread Control Block) model.
//!
//! Every thread is accessed through `Cap<Thread>` — the thread cannot be
//! scheduled, paused, or destroyed without the appropriate capability.

use capability::CapObject;

/// Thread scheduling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    /// Runnable or currently running.
    Running,
    /// Blocked on an IPC endpoint.
    Blocked,
    /// Suspended by the scheduler or a parent.
    Suspended,
    /// Thread has exited.
    Dead,
}

/// A kernel Thread Control Block.
#[derive(Debug)]
pub struct Thread {
    /// Kernel-assigned thread ID.
    pub id: u64,
    /// Human-readable name (for debugging).
    pub name: &'static str,
    /// Current scheduling state.
    pub state: ThreadState,
    /// Priority (0 = lowest, 255 = highest).
    pub priority: u8,
}

impl Thread {
    /// Create a new thread in `Suspended` state.
    pub fn new(id: u64, name: &'static str, priority: u8) -> Self {
        Self { id, name, state: ThreadState::Suspended, priority }
    }

    /// Resume a suspended thread.
    pub fn resume(&mut self) {
        if self.state == ThreadState::Suspended {
            self.state = ThreadState::Running;
        }
    }

    /// Suspend a running thread.
    pub fn suspend(&mut self) {
        if self.state == ThreadState::Running {
            self.state = ThreadState::Suspended;
        }
    }
}

impl CapObject for Thread {
    fn on_revoke(&mut self) {
        self.state = ThreadState::Dead;
    }
}
