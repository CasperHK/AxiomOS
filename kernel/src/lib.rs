//! AxiomOS kernel — top-level entry and subsystem init.
//!
//! This is a host-build stub that wires together all kernel subsystems
//! for integration testing.  Real bare-metal entry lives in `arch/`.

// For the host build we allow std; on real hardware this would be #![no_std].
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

pub mod init;
pub mod syscall;
pub mod thread;
