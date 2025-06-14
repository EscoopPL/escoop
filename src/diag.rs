#![deny(missing_docs)]
//! Implementation of `rustc`'s `Diag` diagnostic system.

use std::sync::OnceLock;

static BUG_FOUND: OnceLock<()> = OnceLock::new();

/// Will return true if there is a bug found in the compiler. Useful for testing.
pub fn bug() -> bool {
    BUG_FOUND.get().is_some()
}

pub(crate) fn set_bug() {
    let _ = BUG_FOUND.set(()); // If there's already a bug found, we don't care.
}