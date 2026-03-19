use super::sample_diff::DIFF;
use crate::{
    cli,
    color::ColorMode,
    config, delta,
    env::DeltaEnv,
    errors::{Error, Result},
    git_config,
    options::get::get_themes,
    utils::bat::output::{OutputType, PagingMode},
};
use bytelines::ByteLines;
use std::{
    convert::TryFrom as _,
    io::{self, BufReader, ErrorKind, IsTerminal, Read},
};

pub fn show_themes(dark: bool, light: bool, color_mode: ColorMode) -> Result<()> {
    let env = DeltaEnv::default();
    let themes = get_themes(git_config::GitConfig::try_create(&env)?);
    if themes.is_empty() {
        Err(Error::NoThemes)?
    }

    let mut input = DIFF.to_vec();

    if !io::stdin().is_terminal() {
        let mut buf = Vec::new();
        io::stdin().lock().read_to_end(&mut buf)?;
        if !buf.is_empty() {
            input = buf;
        }
    };

    let git_config = git_config::GitConfig::try_create(&env)?;
    let opt = cli::Opt::from_iter_and_git_config(
        &env,
        &["delta", "--navigate", "--show-themes"],
        git_config,
    )?;

    let mut output_type = OutputType::from_mode(
        &env,
        PagingMode::Always,
        None,
        &config::Config::try_from(opt)?.into(),
    )
    .unwrap();
    let title_style = ansi_term::Style::new().bold();
    let writer = output_type.handle().unwrap();

    for theme in &themes {
        let git_config = git_config::GitConfig::try_create(&env)?;
        let opt =
            cli::Opt::from_iter_and_git_config(&env, &["delta", "--features", theme], git_config)?;
        let is_dark_theme = opt.dark;
        let is_light_theme = opt.light;
        let config = config::Config::try_from(opt)?;

        if (color_mode == ColorMode::Dark && is_dark_theme)
            || (color_mode == ColorMode::Light && is_light_theme)
            || (dark && light)
        {
            writeln!(writer, "\n\nTheme: {}\n", title_style.paint(theme))?;

            if let Err(Error::Io(error)) =
                delta::delta(ByteLines::new(BufReader::new(&input[0..])), writer, &config)
            {
                match error.kind() {
                    ErrorKind::BrokenPipe => std::process::exit(0),
                    _ => eprintln!("{error}"),
                }
            }
        }
    }

    Ok(())
}
