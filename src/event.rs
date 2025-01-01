use std::path::Path;

pub enum MonitorEvent<'a> {
    DirectoryFound(&'a Path),
    FileFound(&'a Path),
}
