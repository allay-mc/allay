use log::LevelFilter;
use std::{fmt::Write, io, time::SystemTime};

pub(crate) fn setup_logging(verbosity: u8) -> Result<(), fern::InitError> {
    // TODO: log files related to a project should be in project's log dir
    // TODO: only log allay messages

    let level = match verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => unreachable!("invalid verbosity"),
    };

    let session_time = humantime::format_rfc3339(SystemTime::now());
    let colors = fern::colors::ColoredLevelConfig::new()
        .trace(fern::colors::Color::White)
        .debug(fern::colors::Color::Blue)
        .info(fern::colors::Color::Green)
        .warn(fern::colors::Color::Yellow)
        .error(fern::colors::Color::Red);

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let time = SystemTime::now();
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(time),
                record.level(),
                record.target(),
                message,
            ))
        })
        .chain(fern::log_file(
            allay::paths::global::logs().join(format!("{}.log", session_time)),
        )?);

    // TODO: log errors and warnings to stderr
    let console_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{label} {path}:{line} {message}",
                label = styled_label(&colors, &record.level()),
                path = record.module_path().expect("cannot infer module path"),
                line = record.line().expect("cannot infer line"),
                message = message,
            ))
        })
        .chain(io::stdout());

    fern::Dispatch::new()
        .level(level)
        .chain(file_config)
        .chain(console_config)
        .apply()?;

    Ok(())
}

fn styled_label(color_config: &fern::colors::ColoredLevelConfig, level: &log::Level) -> String {
    const MAX_LABEL_WIDTH: usize = "DEBUG".len();
    let color = color_config.get_color(level);
    let padding = " ";
    let extra_padding = " ".repeat(MAX_LABEL_WIDTH - level.as_str().len());
    let mut result = String::new();
    let _ = write!(result, "\x1b[{}m", color.to_bg_str()); // bg color
    let _ = write!(result, "{}", padding);
    let _ = write!(result, "\x1b[1m"); // bold
    let _ = write!(result, "\x1b[{}m", fern::colors::Color::Black.to_fg_str()); // fg color
    let _ = write!(result, "{}", level);
    let _ = write!(result, "{}", padding);
    let _ = write!(result, "\x1b[0m"); // reset style
    let _ = write!(result, "{}", extra_padding);
    result
}
