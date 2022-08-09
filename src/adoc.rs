use chrono::{DateTime, Local};

trait FileInfo {
    fn name(self: &Self) -> &str;
    fn update_time(self: &Self) -> &DateTime<Local>;
    fn create_time(self: &Self) -> &DateTime<Local>;
}
