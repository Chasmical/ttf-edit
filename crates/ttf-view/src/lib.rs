#![feature(const_trait_impl)]
#![feature(const_result_trait_fn)]
#![feature(const_slice_make_iter)]
#![feature(const_option_ops)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_clone)]
#![feature(const_index)]
#![feature(const_iter)]
#![feature(const_cmp)]
#![feature(const_try)]
#![feature(derive_const)]
#![feature(try_trait_v2)]
#![feature(debug_closure_helpers)]
#![feature(formatting_options)]
#![feature(never_type)]
#![feature(bstr)]
#![allow(clippy::missing_safety_doc)] // TODO: remove when adding docs

pub mod platform;
pub mod tables;
pub mod types;
