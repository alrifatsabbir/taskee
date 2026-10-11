use crate::monitor::SystemMonitor;
use crate::ui::theme::*;
use egui::{Align, Layout, ProgressBar, RichText, Ui, Vec2};

pub fn render(ui: &mut Ui, monitor: &SystemMonitor) {
    ui.add_space(2.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("●").color(STATUS_OK).size(12.0));
            ui.label(
                RichText::new(format!("OS: {} {}", monitor.specs.os_name, monitor.specs.os_version))
                    .color(TEXT_MAIN)
                    .strong()
                    .monospace(),
            );
            ui.label(RichText::new("|").color(BORDER));
            ui.label(
                RichText::new(format!("HOST: {}", monitor.specs.host_name))
                    .color(TEXT_MUTED)
                    .monospace(),
            );
            ui.label(RichText::new("|").color(BORDER));
            ui.label(
                RichText::new(format!("KERNEL: {}", monitor.specs.kernel_version))
                    .color(TEXT_MUTED)
                    .monospace(),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("UPTIME: {}", monitor.specs.uptime_formatted))
                        .color(ACCENT_BLUE)
                        .monospace()
                        .strong(),
                );
            });
        });

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label(
                RichText::new(format!("CPU: {}", monitor.specs.cpu_brand))
                    .color(TEXT_MUTED)
                    .monospace()
                    .size(11.0),
            );
            ui.label(RichText::new("|").color(BORDER));
            ui.label(
                RichText::new(format!("{} THREADS", monitor.specs.core_count))
                    .color(ACCENT_TEAL)
                    .monospace()
                    .size(11.0),
            );
            ui.label(RichText::new("|").color(BORDER));
            ui.label(
                RichText::new(format!("RAM TOTAL: {} MB", monitor.total_memory_mb))
                    .color(TEXT_MUTED)
                    .monospace()
                    .size(11.0),
            );
            ui.label(RichText::new("|").color(BORDER));
            ui.label(
                RichText::new(format!("SWAP TOTAL: {} MB", monitor.total_swap_mb))
                    .color(TEXT_MUTED)
                    .monospace()
                    .size(11.0),
            );
        });
    });

    ui.add_space(6.0);

    ui.columns(4, |cols| {
        cols[0].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ CPU_LOAD ]").color(ACCENT_BLUE).size(11.0).monospace().strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}%", monitor.cpu_usage)).color(ACCENT_BLUE).size(13.0).monospace().strong());
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("{:.1}%", monitor.cpu_usage))
                        .color(TEXT_MAIN)
                        .size(24.0)
                        .strong()
                        .monospace(),
                );
                ui.add_space(6.0);
                let progress = (monitor.cpu_usage / 100.0).clamp(0.0, 1.0);
                ui.add(
                    ProgressBar::new(progress)
                        .fill(ACCENT_BLUE)
                        .animate(false),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{} Cores Online", monitor.specs.core_count))
                        .color(TEXT_MUTED)
                        .size(11.0)
                        .monospace(),
                );
            });
        });

        cols[1].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ MEMORY_RAM ]").color(STATUS_OK).size(11.0).monospace().strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}%", monitor.memory_usage_pct)).color(STATUS_OK).size(13.0).monospace().strong());
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("{:.1}%", monitor.memory_usage_pct))
                        .color(TEXT_MAIN)
                        .size(24.0)
                        .strong()
                        .monospace(),
                );
                ui.add_space(6.0);
                let progress = (monitor.memory_usage_pct / 100.0).clamp(0.0, 1.0);
                ui.add(
                    ProgressBar::new(progress)
                        .fill(STATUS_OK)
                        .animate(false),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{}/{} MB", monitor.used_memory_mb, monitor.total_memory_mb))
                        .color(TEXT_MUTED)
                        .size(11.0)
                        .monospace(),
                );
            });
        });

        cols[2].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ SWAP_SPACE ]").color(ACCENT_PURPLE).size(11.0).monospace().strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new(format!("{:.1}%", monitor.swap_usage_pct)).color(ACCENT_PURPLE).size(13.0).monospace().strong());
                    });
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("{:.1}%", monitor.swap_usage_pct))
                        .color(TEXT_MAIN)
                        .size(24.0)
                        .strong()
                        .monospace(),
                );
                ui.add_space(6.0);
                let progress = (monitor.swap_usage_pct / 100.0).clamp(0.0, 1.0);
                ui.add(
                    ProgressBar::new(progress)
                        .fill(ACCENT_PURPLE)
                        .animate(false),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new(format!("{}/{} MB", monitor.used_swap_mb, monitor.total_swap_mb))
                        .color(TEXT_MUTED)
                        .size(11.0)
                        .monospace(),
                );
            });
        });

        cols[3].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ NETWORK_THROUGHPUT ]").color(ACCENT_TEAL).size(11.0).monospace().strong());
                });
                ui.add_space(6.0);
                let total_rx_rate: f64 = monitor.networks.iter().map(|n| n.rx_rate_kbps).sum();
                let total_tx_rate: f64 = monitor.networks.iter().map(|n| n.tx_rate_kbps).sum();

                ui.label(
                    RichText::new(format!("↓ {:.1} KB/s", total_rx_rate))
                        .color(TEXT_MAIN)
                        .size(18.0)
                        .strong()
                        .monospace(),
                );
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!("↑ {:.1} KB/s", total_tx_rate))
                        .color(TEXT_MUTED)
                        .size(13.0)
                        .monospace(),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(format!("{} Network Adapters", monitor.networks.len()))
                        .color(TEXT_MUTED)
                        .size(11.0)
                        .monospace(),
                );
            });
        });
    });

    ui.add_space(6.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("[ STORAGE_VOLUMES ]").color(ACCENT_AMBER).size(12.0).monospace().strong());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(format!("{} Physical Volumes", monitor.disks.len())).color(TEXT_MUTED).size(11.0).monospace());
            });
        });
        ui.add_space(6.0);

        egui::Grid::new("overview_storage_grid")
            .striped(true)
            .min_col_width(70.0)
            .spacing([18.0, 6.0])
            .show(ui, |ui| {
                ui.label(RichText::new("MOUNT POINT").color(TEXT_MUTED).size(11.0).monospace());
                ui.label(RichText::new("FS").color(TEXT_MUTED).size(11.0).monospace());
                ui.label(RichText::new("CAPACITY").color(TEXT_MUTED).size(11.0).monospace());
                ui.label(RichText::new("USAGE").color(TEXT_MUTED).size(11.0).monospace());
                ui.label(RichText::new("VISUAL LOAD").color(TEXT_MUTED).size(11.0).monospace());
                ui.end_row();

                for disk in monitor.disks.iter().take(4) {
                    ui.label(RichText::new(&disk.mount_point).color(TEXT_MAIN).strong().monospace());
                    ui.label(RichText::new(&disk.file_system).color(ACCENT_AMBER).size(11.0).monospace());

                    let total_gb = disk.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let used_gb = disk.used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    ui.label(RichText::new(format!("{:.1} / {:.1} GB", used_gb, total_gb)).color(TEXT_MUTED).monospace());

                    ui.label(RichText::new(format!("{:.1}%", disk.usage_pct)).color(TEXT_MAIN).strong().monospace());

                    let disk_prog = (disk.usage_pct / 100.0).clamp(0.0, 1.0);
                    let bar_color = if disk.usage_pct > 85.0 {
                        ACCENT_RED
                    } else if disk.usage_pct > 65.0 {
                        ACCENT_AMBER
                    } else {
                        ACCENT_BLUE
                    };
                    ui.add_sized(
                        Vec2::new(180.0, 12.0),
                        ProgressBar::new(disk_prog).fill(bar_color).animate(false),
                    );
                    ui.end_row();
                }
            });
    });

    ui.add_space(6.0);

    ui.columns(2, |cols| {
        cols[0].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("[ TOP_CPU_CONSUMERS ]").color(ACCENT_BLUE).size(12.0).monospace().strong());
                ui.add_space(6.0);

                egui::Grid::new("top_cpu_grid")
                    .striped(true)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("PID").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("PROCESS").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("CPU %").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("MEM").color(TEXT_MUTED).size(11.0).monospace());
                        ui.end_row();

                        for p in monitor.processes.iter().take(5) {
                            ui.label(RichText::new(p.pid.to_string()).color(TEXT_MUTED).monospace());
                            ui.label(RichText::new(&p.name).color(TEXT_MAIN).strong().monospace());
                            ui.label(RichText::new(format!("{:.1}%", p.cpu_usage)).color(ACCENT_BLUE).strong().monospace());
                            let mb = p.memory_bytes as f64 / (1024.0 * 1024.0);
                            ui.label(RichText::new(format!("{:.0} MB", mb)).color(TEXT_MUTED).monospace());
                            ui.end_row();
                        }
                    });
            });
        });

        cols[1].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.label(RichText::new("[ TOP_MEMORY_CONSUMERS ]").color(STATUS_OK).size(12.0).monospace().strong());
                ui.add_space(6.0);

                let mut sorted_by_mem = monitor.processes.clone();
                sorted_by_mem.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes));

                egui::Grid::new("top_mem_grid")
                    .striped(true)
                    .spacing([12.0, 6.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("PID").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("PROCESS").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("MEM").color(TEXT_MUTED).size(11.0).monospace());
                        ui.label(RichText::new("MEM %").color(TEXT_MUTED).size(11.0).monospace());
                        ui.end_row();

                        for p in sorted_by_mem.iter().take(5) {
                            ui.label(RichText::new(p.pid.to_string()).color(TEXT_MUTED).monospace());
                            ui.label(RichText::new(&p.name).color(TEXT_MAIN).strong().monospace());
                            let mb = p.memory_bytes as f64 / (1024.0 * 1024.0);
                            ui.label(RichText::new(format!("{:.0} MB", mb)).color(STATUS_OK).strong().monospace());
                            ui.label(RichText::new(format!("{:.1}%", p.memory_pct)).color(TEXT_MUTED).monospace());
                            ui.end_row();
                        }
                    });
            });
        });
    });
}
