//! linux-specific process stats that sysinfo doesn't/can't support

#[cfg(target_os = "linux")]
pub fn get_open_fds(pid: u32) -> Option<usize> {
    std::fs::read_dir(format!("/proc/{pid}/fd"))
        .ok()
        .map(|paths| paths.count())
}

#[cfg(not(target_os = "linux"))]
pub fn get_open_fds(_pid: u32) -> Option<usize> {
    None
}
