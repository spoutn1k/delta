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
    use std::num::{ParseFloatError, ParseIntError};

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error(
            "No themes found. Please see https://dandavison.github.io/delta/custom-themes.html."
        )]
        NoThemes,
        #[error(transparent)]
        Io(#[from] std::io::Error),
        #[error("Not a GitHub, GitLab, SourceHut or Codeberg repo")]
        UnknownGitRemote,
        #[error("Could not parse pager command: {0}")]
        ShellParseError(#[from] shell_words::ParseError),
        #[error("Could not open stdin for pager")]
        NoStdin,
        #[error("Invalid wrap-max-lines argument: {0}")]
        WrapMaxLinesParse(#[from] ParseIntError),
        #[error("Could not parse wrap-right-percent argument: {0}")]
        WrapRightPercentParse(#[from] ParseFloatError),
        #[error("Invalid value for wrap-right-percent: {0}, not between 0 and 100.")]
        WrapRightPercentInvalidValue(f64),
        #[error("Invalid value for {0}, display width of \"{1}\" must be {2} but is {3}")]
        DisplayWidthInvalidValue(String, String, usize, usize),
    }
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
