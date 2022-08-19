#![warn(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::trivially_copy_pass_by_ref,
    clippy::map_unwrap_or,
    clippy::explicit_into_iter_loop,
    clippy::unused_self,
    clippy::needless_pass_by_value
)]

pub mod consumer;
pub mod error;
pub mod from_iter;
pub mod from_mapping;
pub mod into_vec;
pub mod producer;
pub mod result;
