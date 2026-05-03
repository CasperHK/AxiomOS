//! Fixed-size IPC message container.

/// Maximum number of message registers (MRs) per message.
pub const MR_COUNT: usize = 4;

/// A compact descriptor for an IPC message.
///
/// Mirrors seL4's `seL4_MessageInfo_t`:
/// * `label` — caller-defined intent / syscall number.
/// * `length` — number of valid MRs in this message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageInfo {
    /// Caller-defined message label (e.g. syscall number).
    pub label: u64,
    /// Number of valid message registers (0..=`MR_COUNT`).
    pub length: usize,
}

impl MessageInfo {
    /// Construct a `MessageInfo`.
    ///
    /// # Panics
    /// Panics if `length > MR_COUNT`.
    #[inline]
    pub fn new(label: u64, length: usize) -> Self {
        assert!(length <= MR_COUNT, "MessageInfo: length exceeds MR_COUNT");
        Self { label, length }
    }
}

/// A complete IPC message: info + up to [`MR_COUNT`] data registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    /// Descriptor.
    pub info: MessageInfo,
    /// Message register words.
    pub mrs:  [u64; MR_COUNT],
}

impl Message {
    /// Construct an empty message with the given label.
    #[inline]
    pub fn new(label: u64) -> Self {
        Self {
            info: MessageInfo::new(label, 0),
            mrs:  [0u64; MR_COUNT],
        }
    }

    /// Construct a message with `data` written into the first MRs.
    ///
    /// # Panics
    /// Panics if `data.len() > MR_COUNT`.
    pub fn with_data(label: u64, data: &[u64]) -> Self {
        assert!(data.len() <= MR_COUNT, "Message: data exceeds MR_COUNT");
        let mut mrs = [0u64; MR_COUNT];
        mrs[..data.len()].copy_from_slice(data);
        Self {
            info: MessageInfo::new(label, data.len()),
            mrs,
        }
    }

    /// Return the valid data slice.
    #[inline]
    pub fn data(&self) -> &[u64] {
        &self.mrs[..self.info.length]
    }
}
