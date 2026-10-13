extern crate self as synt_extension_api;

mod action_info;
mod adapter;
#[doc(hidden)]
pub mod bindings;
mod error;
mod extension;
mod host;
mod level;
mod register;
mod risk;

pub use action_info::ActionInfo;
#[doc(hidden)]
pub use adapter::Adapter;
pub use error::Error;
pub use extension::Extension;
pub use host::{call, log, storage};
pub use level::Level;
pub use risk::Risk;
