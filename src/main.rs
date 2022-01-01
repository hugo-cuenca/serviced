//! Flexible service manager with dependency support. **CURRENTLY IN DEVELOPMENT**
//!
//! # What?
//! serviced is a simple, yet extensible service manager. This crate currently serves the purpose
//! of reserving the name `serviced` in crates.io, and contains no other code than the standard
//! "Hello, world!".
#![crate_name = "serviced"]
#![cfg_attr(test, deny(warnings))]
#![deny(unused)]
#![deny(unstable_features)]
#![warn(missing_docs)]
#![allow(rustdoc::private_intra_doc_links)]

/// Entry-point for serviced.
fn main() {
    println!("Hello, world!");
}
