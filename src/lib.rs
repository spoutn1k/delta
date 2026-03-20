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
    use crate::{
        handlers::blame::BlameError,
        options::{self, set::SetError},
    };
    use std::{
        collections::HashSet,
        num::{ParseFloatError, ParseIntError},
    };

    pub type Result<T> = std::result::Result<T, Error>;

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("side-by-side requires Some(line_numbers_data)")]
        NoLineNumberData,
        #[error("Number of merge parents must be known.")]
        UnknownParentCount,
        #[error("Failed to find any language syntax definitions.")]
        NoLanguageDefinition,
        #[error("Unreachable code path reached in parse_decoration_style.")]
        InvalidCodePath,
        #[error("Unreachable state: {0}")]
        UnreachableState(String),
        #[error("Unexpected state in new_line_state: {0}")]
        UnexpectedNewLineState(String),
        #[error("Unreachable code reached in get_style.")]
        UnreachableStyle,
        #[error(
            "It looks like you have set delta as the value of $PAGER. \
This would result in a non-terminating recursion. \
delta is not an appropriate value for $PAGER \
(but it is an appropriate value for $GIT_PAGER)."
        )]
        DeltaAsPager,
        #[error("Style not found (git config unavailable): {0}")]
        StyleNotFound(String),
        #[error("Style key not found in git config: {0}")]
        StyleKeyNotFound(String),
        #[error("Your delta styles form a cycle! {0:?}")]
        CyclicalStyles(HashSet<String>),
        #[error("Failed to read git config: {0}")]
        GitConfigError(#[from] git2::Error),
        #[error("Invalid style string: {0}. See the STYLES section of delta --help.")]
        InvalidStyleString(String),
        #[error(
            "You have used the special color 'syntax' as a background color \
             (second color in a style string). It may only be used as a foreground \
             color (first color in a style string)."
        )]
        SyntaxBackground,
        #[error("'{0}' may not be used in a decoration style")]
        DecorationStyleInvalidArgument(String),
        #[error("Invalid option for grep-output-type: Expected \"ripgrep\" or \"classic\".")]
        GrepOutputTypeInvalid,
        #[error("Invalid option for line-fill-method: Expected \"ansi\" or \"spaces\".")]
        LineFillMethodInvalid,
        #[error("Option 'blame-palette' must not be empty.")]
        EmptyBlamePalette,
        #[error(
            "Invalid commit-regex: {0}. \
             The value must be a valid Rust regular expression. \
             See https://docs.rs/regex."
        )]
        CommitRegexInvalid(String),
        #[error(
            "Invalid word-diff-regex: {0}. \
             The value must be a valid Rust regular expression. \
             See https://docs.rs/regex."
        )]
        WordDiffRegexInvalid(String),
        #[error("Invalid color or style attribute: {0}")]
        ColorInvalid(String),
        #[error(
            "No themes found. Please see https://dandavison.github.io/delta/custom-themes.html."
        )]
        NoThemes,
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
        #[error(transparent)]
        SetError(#[from] SetError),
        #[error(transparent)]
        ArgumentParsingError(#[from] clap::Error),
        #[error(transparent)]
        OptionConversion(#[from] options::option_value::ConversionError),
        #[error(transparent)]
        BlameError(#[from] BlameError),
        #[error(transparent)]
        Io(#[from] std::io::Error),
    }
}
