use colored::Colorize;

#[derive(Copy, Clone)]
pub enum Filename {
    Api,
    Library,
    Node,
    Emitter,
    Listener,
    Stream,
    Main,
    Track,
    Tools,
    Reader,

    TrackMetadataSymphonia,
    TrackMetadataMusicBrainz,
    TrackMetadataMP3Metadata,
    TrackMetadataLofty,
    TrackMetadataFilename,
    TrackMetadataDuration,

    XmlLibrary,
}

fn get_filename(filename: Filename) -> String {
    match filename {
        Filename::Api => "API".blue().to_string(),
        Filename::Library => "LIB".blue().to_string(),
        Filename::Node => "COL".blue().to_string(),
        Filename::Emitter => "EMT".cyan().to_string(),
        Filename::Listener => "LST".magenta().to_string(),
        Filename::Stream => "STM".blue().to_string(),
        Filename::Main => "MAI".bold().to_string(),
        Filename::Tools => "TOO".bold().to_string(),
        Filename::Reader => "REA".bold().to_string(),

        Filename::XmlLibrary => "XML".blue().to_string(),

        Filename::Track => format!("AUD").bright_yellow().to_string(),
        Filename::TrackMetadataSymphonia => format!("SYM").bright_yellow().italic().to_string(),
        Filename::TrackMetadataMusicBrainz => format!("MBZ").bright_yellow().italic().to_string(),
        Filename::TrackMetadataMP3Metadata => format!("MP3").bright_yellow().italic().to_string(),
        Filename::TrackMetadataLofty => format!("LOF").bright_yellow().italic().to_string(),
        Filename::TrackMetadataFilename => format!("FIL").bright_yellow().italic().to_string(),
        Filename::TrackMetadataDuration => format!("DUR").bright_yellow().italic().to_string(),
    }
}

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

// LogLevel to numbers
// 0 = Debug, 1 = Info, 2 = Warn, 3 = Error
fn get_log_level(log_level: &LogLevel) -> u8 {
    match log_level {
        LogLevel::Debug => 0,
        LogLevel::Info => 1,
        LogLevel::Warn => 2,
        LogLevel::Error => 3,
    }
}

pub struct Log {
    prefix: Filename,
    lvl: LogLevel,
}

impl Log {
    pub const fn new(prefix: Filename) -> Self {
        Self {
            prefix: prefix,
            lvl: LogLevel::Debug,
        }
    }

    pub const fn new_with_level(prefix: Filename, level: LogLevel) -> Self {
        Self {
            prefix: prefix,
            lvl: level,
        }
    }

    pub fn info(&self, msg: &str) {
        if get_log_level(&self.lvl) > get_log_level(&LogLevel::Info) {
            return;
        }

        let info_node = format!("{}", "INFO".green());
        println!(
            "[{}][{}:{}] {}",
            get_runtime(),
            get_filename(self.prefix),
            info_node,
            msg
        );
    }

    pub fn warn(&self, msg: &str) {
        if get_log_level(&self.lvl) > get_log_level(&LogLevel::Warn) {
            return;
        }

        let warn_node = format!("{}", "WARN".yellow());
        println!(
            "[{}][{}:{}] {}",
            get_runtime(),
            get_filename(self.prefix),
            warn_node,
            msg
        );
    }

    pub fn error(&self, msg: &str) {
        if get_log_level(&self.lvl) > get_log_level(&LogLevel::Error) {
            return;
        }

        let error_node = format!("{}", "ERROR".red());
        println!(
            "[{}][{}:{}] {}",
            get_runtime(),
            get_filename(self.prefix),
            error_node,
            msg
        );
    }

    pub fn debug(&self, msg: &str) {
        if get_log_level(&self.lvl) > get_log_level(&LogLevel::Debug) {
            return;
        }

        let debug_node = format!("{}", "DEBUG".white());
        println!(
            "[{}][{}:{}] {}",
            get_runtime(),
            get_filename(self.prefix),
            debug_node,
            msg
        );
    }
}

// static start_time: SystemTime = SystemTime::now();
use chrono;

fn get_runtime() -> String {
    let local = chrono::Utc::now();
    let local_str = format!(
        "{}:{}",
        local.format("%H:%M:%S"),
        local.timestamp_millis() % 1000
    );

    local_str
}
