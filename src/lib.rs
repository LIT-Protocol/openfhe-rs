//!
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(
    clippy::mod_module_files,
    clippy::unwrap_used,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_parens,
    unused_qualifications
)]

#[macro_use]
mod macros;

mod error;
mod fhe_core;

pub use error::*;
