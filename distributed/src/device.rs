//! Device node — a participant on the distributed soft-bus.

use capability::CapObject;

/// An opaque device identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub u64);

/// Broad classification of devices on the soft-bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    /// Local kernel device (driver running in user-space).
    Local,
    /// Remote device reachable over a network transport.
    Remote,
    /// A virtual device (software emulated).
    Virtual,
}

/// A node in the distributed soft-bus.
///
/// A `Cap<DeviceNode>` is required to send/receive on behalf of this device.
#[derive(Debug)]
pub struct DeviceNode {
    /// Unique device identifier.
    pub id: DeviceId,
    /// Human-readable name (e.g. "camera0", "sensor/imu").
    pub name: &'static str,
    /// Device kind.
    pub kind: DeviceKind,
    /// Whether this device is currently online.
    pub online: bool,
}

impl DeviceNode {
    /// Create a new device node.
    pub fn new(id: u64, name: &'static str, kind: DeviceKind) -> Self {
        Self {
            id: DeviceId(id),
            name,
            kind,
            online: false,
        }
    }

    /// Bring the device online.
    pub fn connect(&mut self) {
        self.online = true;
    }

    /// Take the device offline.
    pub fn disconnect(&mut self) {
        self.online = false;
    }
}

impl CapObject for DeviceNode {
    fn on_revoke(&mut self) {
        self.disconnect();
    }
}
