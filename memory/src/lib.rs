//! # Memory — Memory Objects as Capabilities
//!
//! This crate models seL4's memory object hierarchy in Rust's type system:
//!
//! | seL4 Object         | Rust type            | Description                            |
//! |---------------------|----------------------|----------------------------------------|
//! | Untyped Memory      | [`UntypedMemory`]    | Raw physical memory before retyping    |
//! | Frame (4 KiB page)  | [`Frame`]            | A single mappable physical page        |
//! | PageTable           | [`PageTable`]        | An intermediate page-table level       |
//!
//! Access to all objects is mediated through `Cap<T>` — so possession of a
//! `Cap<Frame>` *is* the authority to map/unmap that page.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod frame;
pub mod untyped;
pub mod page_table;

pub use frame::{Frame, FrameSize};
pub use untyped::UntypedMemory;
pub use page_table::PageTable;
