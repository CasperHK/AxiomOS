//! # Capability System — "Ownership-is-Capability" Model
//!
//! In traditional seL4 (C) a capability is an *index* into a CNode table looked
//! up at runtime.  Here we model each capability as a **First-Class Linear Type**
//! `Cap<T>` whose *ownership* **is** the authority:
//!
//! | seL4 Concept (C)      | Rust Implementation          | Security Benefit                          |
//! |-----------------------|------------------------------|-------------------------------------------|
//! | Capability            | `Cap<T>`                     | Linear type — cannot be copied or cloned  |
//! | CNode slot            | `CNode` / `CapSlot`          | Compile-time slot ownership               |
//! | Rights mask           | `Rights` (bitflags)          | Checked at type + value level             |
//! | Capability derivation | `Cap::derive()`              | Creates child with ⊆ rights               |
//! | Revocation            | `Cap::revoke()` / `drop`     | Rust `Drop` — deterministic revocation    |
//! | Badge                 | `Badge` (newtype `u64`)      | Unforgeable process identity              |
//!
//! ## Design Goals
//! * **No `unsafe` in safe capability paths** — the compiler enforces linearity.
//! * **Minimal TCB** — this crate is `no_std` by default; enable the `std`
//!   feature only for host-side testing.
//! * **Composable rights** — `Rights` is a `bitflags` set; delegation always
//!   produces a rights subset.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod badge;
pub mod cnode;
pub mod error;
pub mod rights;
pub mod cap;

pub use badge::Badge;
pub use cap::{Cap, CapObject};
pub use cnode::{BadgeAllocator, CNode, CapSlot};
pub use error::CapError;
pub use rights::Rights;
