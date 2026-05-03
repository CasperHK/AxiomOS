//! # Distributed — Soft-Bus and Cross-Device Scheduling
//!
//! This crate provides the HarmonyOS-inspired distributed layer on top of the
//! seL4-style capability model:
//!
//! * **[`SoftBus`]** — a logical bus that routes `Message`s between devices
//!   using capability-gated endpoints.
//! * **[`DeviceNode`]** — represents a physical or virtual device on the bus.
//! * **[`ServiceToken`]** — a capability token for a named distributed service.
//!
//! ## Security model
//! Cross-device access is still mediated through `Cap<Endpoint>` — a device can
//! only communicate on the bus if it possesses the appropriate capability.
//! There are no ambient authorities.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod bus;
pub mod device;
pub mod service;

pub use bus::SoftBus;
pub use device::{DeviceId, DeviceNode, DeviceKind};
pub use service::ServiceToken;
