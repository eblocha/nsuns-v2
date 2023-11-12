//! Process metrics

use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use sysinfo::{CpuRefreshKind, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, PidExt};

use super::process_linux::get_open_fds;

pub fn create_system() -> System {
    sysinfo::System::new_with_specifics(
        RefreshKind::new()
            .with_processes(ProcessRefreshKind::new().with_cpu().with_disk_usage())
            .with_cpu(CpuRefreshKind::new().with_cpu_usage())
            .with_memory(),
    )
}

pub fn record_process_metrics(sys: Arc<Mutex<System>>) -> anyhow::Result<()> {
    let mut sys = match sys.lock() {
        Ok(sys) => sys,
        Err(e) => e.into_inner(),
    };

    let pid = sysinfo::get_current_pid().map_err(|e| anyhow!(e))?;

    sys.refresh_process(pid);

    // unlikely to have so many cpus that we can't represent it as an f32
    let n_cpus = sys.cpus().len().max(1) as f32;

    if let Some(process) = sys.process(pid) {
        let du = process.disk_usage();

        metrics::gauge!(
            "process.cpu.utilization",
            (process.cpu_usage() / n_cpus) as f64
        );
        metrics::gauge!("process.memory.usage", process.memory() as f64);
        metrics::gauge!("process.memory.virtual", process.virtual_memory() as f64);
        metrics::counter!("process.disk.io", du.read_bytes, &[("direction", "read")]);
        metrics::counter!("process.disk.io", du.written_bytes, &[("direction", "write")]);
    }

    if let Some(fd_count) = get_open_fds(pid.as_u32()) {
        metrics::gauge!("process.open_file_descriptors", fd_count as f64);
    }

    Ok(())
}
