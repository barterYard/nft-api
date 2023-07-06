use chrono::{offset::Utc, DateTime};
use std::io::Write;
use std::time::SystemTime;

pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            let datetime: DateTime<Utc> = SystemTime::now().into();
            if record.level() == log::Level::Error {
                let file_line = format!(
                    "{:?} {}",
                    record.file().unwrap_or(""),
                    record.line().unwrap_or_default()
                );
                return writeln!(
                    buf,
                    "{} {}: -{}- {}",
                    datetime.format("%T %D"),
                    record.level(),
                    file_line,
                    record.args()
                );
            }

            writeln!(
                buf,
                "{} {}: {}",
                datetime.format("%T %D"),
                record.level(),
                record.args()
            )
        })
        .init();
}

pub fn get_log_format() -> &'static str {
    "%s %r - %{r}a %Dms"
}
