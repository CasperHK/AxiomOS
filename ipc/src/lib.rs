//! # IPC — Inter-Process Communication as Capabilities
//!
//! This crate implements seL4-style synchronous IPC where *endpoints* and
//! *reply capabilities* are first-class `Cap<T>` values.
//!
//! ## IPC model
//! ```text
//!  Client                               Server
//!  ──────                               ──────
//!  cap: Cap<Endpoint>                   ep: Cap<Endpoint>
//!        │                                      │
//!        │  Call(msg, reply_cap)                │
//!        ├──────────────────────────────────────►
//!        │                              Recv → (msg, reply_cap)
//!        │                              ... process ...
//!        │                              ReplyRecv(reply_cap, resp)
//!        ◄──────────────────────────────────────┤
//! ```
//!
//! ## Key types
//! * [`Endpoint`] — a kernel IPC rendezvous point, wrapped in `Cap<Endpoint>`.
//! * [`Message`]  — a fixed-size message (info word + message registers).
//! * [`ReplyObj`] — a one-shot reply capability object.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod endpoint;
pub mod message;
pub mod reply;

pub use endpoint::{Endpoint, EndpointState};
pub use message::{Message, MessageInfo, MR_COUNT};
pub use reply::ReplyObj;
