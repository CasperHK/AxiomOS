//! Page-table kernel object.

use capability::CapObject;

/// A single level of the hardware page-table hierarchy.
///
/// On AArch64 / RISC-V this corresponds to one VSpace-level node.
/// `Cap<PageTable>` represents the authority to install/remove mappings
/// within this table level.
#[derive(Debug)]
pub struct PageTable {
    /// Physical address of the page-table page.
    pub phys_addr: u64,
    /// Number of installed entries.
    pub entry_count: usize,
    /// Whether this table is currently installed in a VSpace.
    pub installed: bool,
}

impl PageTable {
    /// Allocate a page-table at the given physical address.
    pub fn new(phys_addr: u64) -> Self {
        Self { phys_addr, entry_count: 0, installed: false }
    }

    /// Install this page-table into the active VSpace.
    pub fn install(&mut self) {
        self.installed = true;
    }

    /// Uninstall this page-table from the active VSpace.
    pub fn uninstall(&mut self) {
        self.installed = false;
    }
}

impl CapObject for PageTable {
    fn on_revoke(&mut self) {
        self.uninstall();
    }
}
