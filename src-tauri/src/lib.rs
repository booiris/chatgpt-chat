use dal::init_dal;
use log::{error, LevelFilter};
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{policy, RollingFileAppender},
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use tauri::{App, AppHandle};

mod dal;
mod handler;
mod js_handler;
mod model;

use js_handler::*;
use std::{panic, path::PathBuf};

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

fn init_log(log_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})}: {d(%Y-%m-%d %H:%M:%S)} - {f} - {m}{n}",
        )))
        .build();

    let back_log_dir = format!("{}/compressed-log-{{}}.log", log_dir.display());
    let roller = policy::compound::roll::fixed_window::FixedWindowRollerBuilder::default()
        .build(&back_log_dir, 10)?;
    let trigger = policy::compound::trigger::size::SizeTrigger::new(10 * 1024 * 1024);
    let policy = policy::compound::CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let log_dir = format!("{}/log.log", log_dir.display());
    let file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})}: {d(%Y-%m-%d %H:%M:%S)} - {f} - {m}{n}",
        )))
        .append(false)
        .build(&log_dir, Box::new(policy))?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appenders(["stdout", "file"])
                .build(LevelFilter::Info),
        )?;
    // .logger(
    //     Logger::builder()
    //         .appender("file")
    //         .additive(false)
    //         .build("app", LevelFilter::Info),
    // )
    log4rs::init_config(config)?;
    Ok(())
}

fn init(app: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(move |panic_info| {
        error!("{:?}", panic_info);
    }));
    match app.path_resolver().app_log_dir() {
        Some(log_dir) => init_log(log_dir)?,
        _ => {
            // return Err("log dir not found".into());
            crate::dal::debug_print::init::debug_print("log dir not found");
        }
    }
    init_dal()?;
    Ok(())
}

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {
        let setup = self.setup;
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![query, get_dir, init_backend])
            .setup(move |app| {
                {
                    use dal::debug_print::init::init_debug_print;
                    let app_handler = app.handle();
                    std::thread::spawn(move || {
                        init_debug_print(app_handler);
                    });
                }

                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
