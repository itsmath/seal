#![feature(default_field_values)]
pub use crate::{prelude::*, setup::SetupOptions};
pub use mluau::prelude::*;

pub use std::env;
pub use std::ffi::OsString;
pub use std::collections::VecDeque;
pub use std::fs::{self, OpenOptions};
pub use std::io::Write;

pub mod prelude;
pub mod std_env;
pub mod std_fs;
pub mod std_json;
pub mod std_process;
pub mod std_time;
pub mod table_helpers;
#[macro_use]
pub mod err;
pub mod globals;
pub mod interop;
pub mod require;
pub mod std_crypt;
pub mod std_io;
pub mod std_net;
pub mod std_serde;
pub mod std_str_internal;
pub mod std_thread;
pub mod std_luau;
pub mod std_err;
pub mod sealconfig;
pub mod setup;
pub mod compile;

pub use err::display_error_and_exit;
pub use sealconfig::SealConfig;
pub use globals::SEAL_VERSION;
