mod align;
pub mod ansi;
pub mod cli;
mod color;
mod colors;
pub mod config;
pub mod delta;
mod edits;
pub mod env;
mod features;
mod format;
mod git_config;
mod handlers;
mod minusplus;
mod options;
mod paint;
mod parse_style;
mod parse_styles;
mod style;
pub mod utils;
mod wrapping;

pub mod subcommands;

mod tests;

pub mod errors {
    pub use anyhow::{Context, Error, Result, anyhow};
}

pub fn fatal<T>(errmsg: T) -> !
where
    T: AsRef<str> + std::fmt::Display,
{
    #[cfg(not(test))]
    {
        use std::process;

        eprintln!("{errmsg}");
        // As in Config::error_exit_code: use 2 for error
        // because diff uses 0 and 1 for non-error.
        process::exit(2);
    }
    #[cfg(test)]
    panic!("{}\n", errmsg);
}
