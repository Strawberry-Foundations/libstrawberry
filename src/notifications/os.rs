#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum OS {
    Undefined,
    Unknown,
    Linux,
    LinuxLibNotify,
    Windows,
    WindowsLegacy,
    MacOS,
}
