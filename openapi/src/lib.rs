#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate paperclip_macros;
#[macro_use]
extern crate serde_derive;

pub mod error;
#[cfg(feature = "v2")]
pub mod v2;
