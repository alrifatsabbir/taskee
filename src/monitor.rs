#![allow(dead_code)]

use std::collections::VecDeque;
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;
use sysinfo::{Disks, Networks, Pid, System};

#[derive(Clone, Debug)]
pub struct CpuCoreInfo {
    pub name: String,
    pub usage: f32,
    pub frequency_mhz: u64,
}

#[derive(Clone, Debug)]
pub struct SystemSpecs {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub host_name: String,
    pub cpu_brand: String,
    pub core_count: usize,
    pub uptime_formatted: String,
}

#[derive(Clone, Debug)]
pub struct DiskPartition {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
    pub usage_pct: f32,
}

#[derive(Clone, Debug)]
pub struct NetworkStat {
    pub interface_name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_rate_kbps: f64,
    pub tx_rate_kbps: f64,
}

#[derive(Clone, Debug)]
pub struct ProcessItem {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub memory_pct: f32,
    pub virtual_memory_bytes: u64,
    pub status: String,
    pub run_time_secs: u64,
}

pub struct SystemMonitor {
    sys: System,
    pub specs: SystemSpecs,
    pub cpu_usage: f32,
    pub cores: Vec<CpuCoreInfo>,
    pub used_memory_mb: u64,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub memory_usage_pct: f32,
    pub used_swap_mb: u64,
    pub total_swap_mb: u64,
    pub swap_usage_pct: f32,
    pub disks: Vec<DiskPartition>,
    pub networks: Vec<NetworkStat>,
    pub processes: Vec<ProcessItem>,
    pub running_processes_count: usize,
    pub sleeping_processes_count: usize,
    pub cpu_history: VecDeque<[f64; 2]>,
    pub memory_history: VecDeque<[f64; 2]>,
    pub elapsed_seconds: f64,
    last_net_totals: (u64, u64),
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    let s = secs % 60;
    if days > 0 {
        format!("{}d {:02}h {:02}m {:02}s", days, hours, mins, s)
    } else {
        format!("{:02}h {:02}m {:02}s", hours, mins, s)
    }
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        thread::sleep(Duration::from_millis(250));
        sys.refresh_all();

        let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
        let os_ver = System::os_version().unwrap_or_else(|| "".to_string());
        let kernel = System::kernel_version().unwrap_or_else(|| "".to_string());
        let host = System::host_name().unwrap_or_else(|| "Localhost".to_string());
        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().trim().to_string())
            .unwrap_or_else(|| "Generic CPU".to_string());
        let core_count = sys.cpus().len();

        let specs = SystemSpecs {
            os_name,
            os_version: os_ver,
            kernel_version: kernel,
            host_name: host,
            cpu_brand,
            core_count,
            uptime_formatted: format_uptime(System::uptime()),
        };

        let mut monitor = Self {
            sys,
            specs,
            cpu_usage: 0.0,
            cores: Vec::new(),
            used_memory_mb: 0,
            total_memory_mb: 0,
            available_memory_mb: 0,
            memory_usage_pct: 0.0,
            used_swap_mb: 0,
            total_swap_mb: 0,
            swap_usage_pct: 0.0,
            disks: Vec::new(),
            networks: Vec::new(),
            processes: Vec::new(),
            running_processes_count: 0,
            sleeping_processes_count: 0,
            cpu_history: VecDeque::with_capacity(120),
            memory_history: VecDeque::with_capacity(120),
            elapsed_seconds: 0.0,
            last_net_totals: (0, 0),
        };

        monitor.refresh_internal();
        monitor.cpu_history.push_back([0.0, monitor.cpu_usage as f64]);
        monitor.memory_history.push_back([0.0, monitor.memory_usage_pct as f64]);
        monitor
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.elapsed_seconds += 1.0;
        self.refresh_internal();

        self.cpu_history.push_back([self.elapsed_seconds, self.cpu_usage as f64]);
        self.memory_history.push_back([self.elapsed_seconds, self.memory_usage_pct as f64]);

        while let Some(front) = self.cpu_history.front() {
            if self.elapsed_seconds - front[0] > 60.0 {
                self.cpu_history.pop_front();
            } else {
                break;
            }
        }

        while let Some(front) = self.memory_history.front() {
            if self.elapsed_seconds - front[0] > 60.0 {
                self.memory_history.pop_front();
            } else {
                break;
            }
        }
    }

    fn refresh_internal(&mut self) {
        self.specs.uptime_formatted = format_uptime(System::uptime());
        self.cpu_usage = self.sys.global_cpu_usage();

        self.cores = self
            .sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(idx, cpu)| CpuCoreInfo {
                name: format!("Core {:02}", idx),
                usage: cpu.cpu_usage(),
                frequency_mhz: cpu.frequency(),
            })
            .collect();

        let total_mem = self.sys.total_memory();
        let used_mem = self.sys.used_memory();
        let avail_mem = self.sys.available_memory();
        self.total_memory_mb = total_mem / (1024 * 1024);
        self.used_memory_mb = used_mem / (1024 * 1024);
        self.available_memory_mb = avail_mem / (1024 * 1024);
        self.memory_usage_pct = if total_mem > 0 {
            (used_mem as f32 / total_mem as f32) * 100.0
        } else {
            0.0
        };

        let total_swap = self.sys.total_swap();
        let used_swap = self.sys.used_swap();
        self.total_swap_mb = total_swap / (1024 * 1024);
        self.used_swap_mb = used_swap / (1024 * 1024);
        self.swap_usage_pct = if total_swap > 0 {
            (used_swap as f32 / total_swap as f32) * 100.0
        } else {
            0.0
        };

        let disks = Disks::new_with_refreshed_list();
        let mut disk_list = Vec::new();
        for d in &disks {
            let total = d.total_space();
            if total < 100 * 1024 * 1024 {
                continue;
            }
            let name = OsStr::new(d.name()).to_string_lossy().to_string();
            let mount = OsStr::new(d.mount_point()).to_string_lossy().to_string();
            let fs = OsStr::new(d.file_system()).to_string_lossy().to_string();
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = if total > 0 {
                (used as f32 / total as f32) * 100.0
            } else {
                0.0
            };
            disk_list.push(DiskPartition {
                name,
                mount_point: mount,
                file_system: fs,
                total_bytes: total,
                available_bytes: avail,
                used_bytes: used,
                usage_pct: pct,
            });
        }
        disk_list.sort_by(|a, b| b.total_bytes.cmp(&a.total_bytes));
        self.disks = disk_list;

        let networks = Networks::new_with_refreshed_list();
        let mut cur_rx_sum = 0u64;
        let mut cur_tx_sum = 0u64;
        let mut net_list = Vec::new();

        for (iface, net) in &networks {
            let rx = net.received();
            let tx = net.transmitted();
            cur_rx_sum += rx;
            cur_tx_sum += tx;
            net_list.push(NetworkStat {
                interface_name: iface.clone(),
                rx_bytes: rx,
                tx_bytes: tx,
                rx_rate_kbps: 0.0,
                tx_rate_kbps: 0.0,
            });
        }

        let delta_rx = cur_rx_sum.saturating_sub(self.last_net_totals.0);
        let delta_tx = cur_tx_sum.saturating_sub(self.last_net_totals.1);
        self.last_net_totals = (cur_rx_sum, cur_tx_sum);

        for n in &mut net_list {
            n.rx_rate_kbps = (delta_rx as f64) / 1024.0;
            n.tx_rate_kbps = (delta_tx as f64) / 1024.0;
        }
        self.networks = net_list;

        let num_cores = self.sys.cpus().len().max(1) as f32;
        let mut list = Vec::new();
        let mut running = 0usize;
        let mut sleeping = 0usize;

        for (pid, proc) in self.sys.processes() {
            let status_str = format!("{:?}", proc.status());
            if status_str.contains("Run") {
                running += 1;
            } else {
                sleeping += 1;
            }

            let mem = proc.memory();
            let mem_pct = if total_mem > 0 {
                (mem as f32 / total_mem as f32) * 100.0
            } else {
                0.0
            };

            let normalized_cpu = (proc.cpu_usage() / num_cores).clamp(0.0, 100.0);

            list.push(ProcessItem {
                pid: pid.as_u32(),
                name: OsStr::new(proc.name()).to_string_lossy().to_string(),
                cpu_usage: normalized_cpu,
                memory_bytes: mem,
                memory_pct: mem_pct,
                virtual_memory_bytes: proc.virtual_memory(),
                status: status_str,
                run_time_secs: proc.run_time(),
            });
        }

        list.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        self.processes = list;
        self.running_processes_count = running;
        self.sleeping_processes_count = sleeping;
    }

    pub fn kill_process(&mut self, pid: u32) -> bool {
        let target_pid = Pid::from_u32(pid);
        if let Some(proc) = self.sys.process(target_pid) {
            let res = proc.kill();
            self.refresh();
            res
        } else {
            false
        }
    }
}
