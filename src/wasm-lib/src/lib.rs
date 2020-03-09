#![crate_type="lib"]
#![crate_name="wasmlib"]

#[macro_use]
extern crate lazy_static;

pub mod host_api;
pub use host_api::*;

pub mod resource;
pub use resource::*;
