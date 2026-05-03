//! Service token — capability for a named distributed service.

use capability::CapObject;

/// A token representing authority to invoke a named distributed service.
///
/// Possession of `Cap<ServiceToken>` proves the holder was explicitly granted
/// access by the service owner.  There is no global service registry accessible
/// without a capability.
#[derive(Debug)]
pub struct ServiceToken {
    /// Service name (e.g. "audio.playback", "net.dhcp").
    pub name: &'static str,
    /// Unique service instance ID assigned at registration.
    pub instance_id: u64,
}

impl ServiceToken {
    /// Create a new service token.
    pub fn new(name: &'static str, instance_id: u64) -> Self {
        Self { name, instance_id }
    }
}

impl CapObject for ServiceToken {
    fn on_revoke(&mut self) {
        // Service would be unregistered from the bus here.
    }
}
