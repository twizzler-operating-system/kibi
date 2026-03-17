// SPDX-FileCopyrightText: 2020 Ilaï Deutel & Kibi Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Kibi
//!
//! Kibi is a text editor in ≤1024 lines of code.

pub use crate::{config::Config, editor::run, error::Error, sys::stdin};

pub mod ansi_escape;
mod config;
mod editor;
mod error;
mod row;
mod syntax;
mod terminal;

#[cfg_attr(windows, path = "windows.rs")]
#[cfg_attr(any(unix, target_os = "twizzler"), path = "unix.rs")]
#[cfg_attr(target_os = "wasi", path = "wasi.rs")]
mod sys;

#[cfg(any(unix, target_os = "wasi", target_os = "twizzler"))]
mod xdg;
