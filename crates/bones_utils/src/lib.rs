//! General utilities for [Bones] meta-engine crates.
//!
//! [Bones]: https://fishfolk.org/development/bones/introduction/
//!
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]
#![warn(clippy::undocumented_unsafe_blocks)]

mod collections;
mod default;
mod key;
mod labeled_id;
mod names;
mod ptr;

/// Helper to export the same types in the crate root and in the prelude.
macro_rules! pub_use {
    () => {
        pub use crate::{collections::*, default::*, key::*, labeled_id::*, names::*, ptr::*};
        pub use bevy_ptr::*;
        pub use bones_utils_macros::*;
        pub use hashbrown;
        pub use parking_lot;
    };
}
pub_use!();

/// The prelude.
pub mod prelude {
    pub_use!();
}
