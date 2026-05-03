//! Kernel boot / initialisation sequence.
//!
//! Mirrors seL4's `init_kernel()`:
//! 1. Bootstrap the root CNode.
//! 2. Retype initial UntypedMemory objects.
//! 3. Create the initial IPC endpoints.
//! 4. Bring up the distributed soft-bus.
//! 5. Hand control to the root task.

use capability::{Badge, Cap, CNode, Rights};
use distributed::{DeviceKind, DeviceNode, SoftBus};
use ipc::Endpoint;
use memory::{Frame, FrameSize, UntypedMemory};

/// Perform the kernel boot sequence and return when the root task exits.
pub fn boot() {
    // ── 1. Root CNode ────────────────────────────────────────────────────────
    let mut root_cnode: CNode<Endpoint, 16> = CNode::new();

    // ── 2. Initial untyped memory ────────────────────────────────────────────
    let mut untyped = UntypedMemory::new(0x1000_0000, 20 /* 1 MiB */);
    assert!(untyped.is_available());
    untyped.consume(); // "retype" into a frame below

    // ── 3. IPC endpoint ──────────────────────────────────────────────────────
    let ep_cap: Cap<Endpoint> = Cap::mint(
        Endpoint::new(1),
        Rights::ALL,
        Badge::NONE,
    );
    root_cnode.insert(0, ep_cap).expect("insert ep into slot 0");
    assert_eq!(root_cnode.occupied_count(), 1);

    // ── 4. Frame capability ──────────────────────────────────────────────────
    let _frame_cap: Cap<Frame> = Cap::mint(
        Frame::new(0x1000_0000, FrameSize::Small),
        Rights::READ | Rights::WRITE,
        Badge::NONE,
    );

    // ── 5. Distributed soft-bus ──────────────────────────────────────────────
    let mut bus = SoftBus::new();
    let mut dev = DeviceNode::new(1, "sensor/imu", DeviceKind::Local);
    assert!(bus.register(&mut dev));
    assert_eq!(bus.device_count(), 1);

    println!("  [boot] root CNode: {} slot(s) occupied", root_cnode.occupied_count());
    println!("  [boot] soft-bus: {} device(s) online", bus.device_count());
    println!("AxiomOS boot complete.");
}
