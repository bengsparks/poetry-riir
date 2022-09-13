#![feature(backtrace, backtrace_frames)]
#![feature(io_error_more)]

// #![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

pub mod add;
pub mod error;
pub mod init;

mod document;
