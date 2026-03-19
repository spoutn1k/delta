use crate::{
    config::{self},
    subcommands::{SubCmdKind, SubCommand},
    utils::git::retrieve_git_version,
};
use std::{ffi::OsString, path::Path};

/// Build `git diff` command for the files provided on the command line. Fall back to
/// `diff` if the supplied "files" use process substitution.
pub fn build_diff_cmd(
    minus_file: &Path,
    plus_file: &Path,
    config: &config::Config,
) -> Result<SubCommand, i32> {
    // suppress `dead_code` warning, values are accessed via `get_one::<PathBuf>("plus/minus_file")`
    debug_assert!(config.minus_file.as_ref().unwrap() == minus_file);
    debug_assert!(config.plus_file.as_ref().unwrap() == plus_file);

    let mut diff_args = match shell_words::split(config.diff_args.trim()) {
        Ok(words) => words,
        Err(err) => {
            eprintln!("Failed to parse diff args: {}: {err}", config.diff_args);
            return Err(config.error_exit_code);
        }
    };
    // Permit e.g. -@U1
    if diff_args
        .first()
        .map(|arg| !arg.is_empty() && !arg.starts_with('-'))
        .unwrap_or(false)
    {
        diff_args[0] = format!("-{}", diff_args[0])
    }

    let via_process_substitution =
        |f: &Path| f.starts_with("/proc/self/fd/") || f.starts_with("/dev/fd/");

    // https://stackoverflow.com/questions/22706714/why-does-git-diff-not-work-with-process-substitution
    // git <2.42 does not support process substitution
    let (differ, mut diff_cmd) = match retrieve_git_version() {
        Some(version)
            if version >= (2, 42)
                || !(via_process_substitution(minus_file)
                    || via_process_substitution(plus_file)) =>
        {
            (
                SubCmdKind::GitDiff,
                vec!["git", "diff", "--no-index", "--color"],
            )
        }
        _ => (
            SubCmdKind::Diff,
            if diff_args_set_unified_context(&diff_args) {
                vec!["diff"]
            } else {
                vec!["diff", "-U3"]
            },
        ),
    };

    diff_cmd.extend(
        diff_args
            .iter()
            .filter(|s| !s.is_empty())
            .map(String::as_str),
    );
    diff_cmd.push("--");
    let mut diff_cmd = diff_cmd.iter().map(OsString::from).collect::<Vec<_>>();
    diff_cmd.push(minus_file.into());
    diff_cmd.push(plus_file.into());
    Ok(SubCommand::new(differ, diff_cmd))
}

/// Do the user-supplied `diff` args set the unified context?
fn diff_args_set_unified_context<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    // This function is applied to `diff` args; not `git diff`.
    for arg in args {
        let arg = arg.as_ref();
        if arg == "-u" || arg == "-U" {
            // diff allows a space after -U (git diff does not)
            return true;
        }
        if (arg.starts_with("-U") || arg.starts_with("-u"))
            && arg.split_at(2).1.parse::<u32>().is_ok()
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod main_tests {
    use super::diff_args_set_unified_context;
    use rstest::rstest;

    #[rstest]
    #[case(&["-u"], true)]
    #[case(&["-u7"], true)]
    #[case(&["-u77"], true)]
    #[case(&["-ux"], false)]
    #[case(&["-U"], true)]
    #[case(&["-U7"], true)]
    #[case(&["-U77"], true)]
    #[case(&["-Ux"], false)]
    fn test_unified_diff_arg_is_detected_in_diff_args(
        #[case] diff_args: &[&str],
        #[case] expected: bool,
    ) {
        assert_eq!(diff_args_set_unified_context(diff_args), expected)
    }
}
