use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};

pub fn setup_logger(log_level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .debug(Color::BrightMagenta)
        .info(Color::BrightBlue)
        .trace(Color::Green)
        .warn(Color::BrightYellow)
        .error(Color::BrightRed);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}/{}] {}",
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
