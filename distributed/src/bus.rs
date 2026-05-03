//! Distributed soft-bus — routes messages between device nodes.

use crate::device::{DeviceId, DeviceNode};
use ipc::message::Message;

/// Maximum number of device nodes the bus can hold.
pub const MAX_DEVICES: usize = 64;

/// A logical soft-bus connecting multiple [`DeviceNode`]s.
///
/// The soft-bus does **not** bypass the capability system: a caller must have a
/// `Cap<DeviceNode>` to register or route through a node.  The bus itself only
/// maintains routing metadata.
pub struct SoftBus {
    /// Routing table: index ↦ (DeviceId, name).
    routes: [(DeviceId, &'static str); MAX_DEVICES],
    /// Number of registered devices.
    count: usize,
}

impl SoftBus {
    /// Create an empty soft-bus.
    pub const fn new() -> Self {
        Self {
            routes: [(DeviceId(0), ""); MAX_DEVICES],
            count: 0,
        }
    }

    /// Register a device on the bus.
    ///
    /// # Errors
    /// Returns `false` if the bus is full.
    pub fn register(&mut self, node: &mut DeviceNode) -> bool {
        if self.count >= MAX_DEVICES {
            return false;
        }
        self.routes[self.count] = (node.id, node.name);
        self.count += 1;
        node.connect();
        true
    }

    /// Deregister a device.
    ///
    /// Returns `true` if the device was found and removed.
    pub fn deregister(&mut self, id: DeviceId) -> bool {
        for i in 0..self.count {
            if self.routes[i].0 == id {
                self.routes[i] = self.routes[self.count - 1];
                self.count -= 1;
                return true;
            }
        }
        false
    }

    /// Return the number of registered devices.
    #[inline]
    pub fn device_count(&self) -> usize {
        self.count
    }

    /// Look up a device name by ID.
    pub fn lookup_name(&self, id: DeviceId) -> Option<&'static str> {
        self.routes[..self.count]
            .iter()
            .find(|(did, _)| *did == id)
            .map(|(_, name)| *name)
    }

    /// Simulate routing a message to the device with `dest_id`.
    ///
    /// Returns `Some(msg)` if the destination is registered, `None` otherwise.
    pub fn route(&self, dest_id: DeviceId, msg: Message) -> Option<Message> {
        if self.routes[..self.count].iter().any(|(did, _)| *did == dest_id) {
            Some(msg)
        } else {
            None
        }
    }
}

impl Default for SoftBus {
    fn default() -> Self {
        Self::new()
    }
}
