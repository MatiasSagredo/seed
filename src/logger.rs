use log::set_logger;
#[cfg(debug_assertions)]
use log::{Level, Log};
use owo_colors::OwoColorize;

struct SeedLogger;

static LOGGER: SeedLogger = SeedLogger;

pub fn init_logger() {
    set_logger(&LOGGER).unwrap();
    #[cfg(debug_assertions)]
    {
        log::set_max_level(log::LevelFilter::Trace);
    }
    #[cfg(not(debug_assertions))]
    {
        log::set_max_level(log::LevelFilter::Info);
    }
}

impl Log for SeedLogger {
    fn flush(&self) {}
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        #[cfg(debug_assertions)]
        {
            metadata.level() <= log::Level::Trace
        }
        #[cfg(not(debug_assertions))]
        {
            metadata.level() <= log::Level::Info
        }
    }
    fn log(&self, record: &log::Record) {
        let level_str = record.level().as_str().to_uppercase();
        let target_str = record.target();
        let args = record.args();
        #[cfg(debug_assertions)]
        {
            let file = record.file_static().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            let fileline = format_args!("{}:{}", file, line);
            let msg = match record.level() {
                Level::Trace | Level::Debug => format_args!(
                    "[{}][{}] {}: {}",
                    level_str.dimmed(),
                    target_str.dimmed(),
                    fileline.dimmed(),
                    args.dimmed()
                ),
                Level::Info => format_args!(
                    "[{}][{}] {}: {}",
                    level_str.cyan(),
                    target_str.white(),
                    fileline.dimmed(),
                    args.cyan()
                ),
                Level::Warn => format_args!(
                    "[{}][{}] {}: {}",
                    level_str.yellow(),
                    target_str.white(),
                    fileline.dimmed(),
                    args.yellow()
                ),
                Level::Error => format_args!(
                    "[{}][{}] {}: {}",
                    level_str.red(),
                    target_str.white(),
                    fileline.dimmed(),
                    args.red()
                ),
            };

            match record.level() {
                Level::Trace | Level::Debug | Level::Info => println!("{}", msg),
                Level::Warn | Level::Error => eprintln!("{}", msg),
            }
        }
        #[cfg(not(debug_assertions))]
        {
            let msg = match record.level() {
                Level::Trace | Level::Debug => format_args!(
                    "[{}][{}]: {}",
                    level_str.dimmed(),
                    target_str.dimmed(),
                    args.dimmed()
                ),
                Level::Info => format_args!(
                    "[{}][{}]: {}",
                    level_str.cyan(),
                    target_str.white(),
                    args.cyan()
                ),
                Level::Warn => format_args!(
                    "[{}][{}]: {}",
                    level_str.yellow(),
                    target_str.white(),
                    args.yellow()
                ),
                Level::Error => format_args!(
                    "[{}][{}]: {}",
                    level_str.red(),
                    target_str.white(),
                    args.red()
                ),
            };
            match record.level() {
                Level::Info => println!("{}", msg),
                Level::Warn | Level::Error => eprintln!("{}", msg),
                _ => {}
            }
        }
    }
}
