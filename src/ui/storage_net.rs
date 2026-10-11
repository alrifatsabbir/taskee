use crate::monitor::SystemMonitor;
use crate::ui::theme::*;
use egui::{Align, Layout, ProgressBar, RichText, ScrollArea, Ui};

fn format_bytes(bytes: u64) -> String {
    let mb = bytes as f64 / (1024.0 * 1024.0);
    if mb >= 1024.0 {
        format!("{:.2} GB", mb / 1024.0)
    } else {
        format!("{:.1} MB", mb)
    }
}

pub fn render(ui: &mut Ui, monitor: &SystemMonitor) {
    ui.add_space(4.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("[ FILESYSTEM_PARTITIONS ]").color(ACCENT_AMBER).size(12.0).monospace().strong());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(format!("{} Devices Mounted", monitor.disks.len())).color(TEXT_MUTED).size(11.0).monospace());
            });
        });
        ui.add_space(6.0);

        ScrollArea::vertical()
            .max_height(240.0)
            .show(ui, |ui| {
                egui::Grid::new("disks_detail_grid")
                    .striped(true)
                    .min_col_width(80.0)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("MOUNT").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("DEVICE").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("FS").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("USED").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("FREE").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("TOTAL").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("USAGE BAR").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.end_row();

                        for d in &monitor.disks {
                            ui.label(RichText::new(&d.mount_point).color(TEXT_MAIN).strong().monospace());
                            ui.label(RichText::new(&d.name).color(TEXT_MUTED).monospace());
                            ui.label(RichText::new(&d.file_system).color(ACCENT_AMBER).monospace());
                            ui.label(RichText::new(format_bytes(d.used_bytes)).color(TEXT_MAIN).monospace());
                            ui.label(RichText::new(format_bytes(d.available_bytes)).color(STATUS_OK).monospace());
                            ui.label(RichText::new(format_bytes(d.total_bytes)).color(TEXT_MUTED).monospace());

                            let prog = (d.usage_pct / 100.0).clamp(0.0, 1.0);
                            let color = if d.usage_pct > 85.0 {
                                ACCENT_RED
                            } else if d.usage_pct > 65.0 {
                                ACCENT_AMBER
                            } else {
                                ACCENT_BLUE
                            };
                            ui.add(ProgressBar::new(prog).fill(color).animate(false));
                            ui.end_row();
                        }
                    });
            });
    });

    ui.add_space(8.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("[ NETWORK_INTERFACES ]").color(ACCENT_TEAL).size(12.0).monospace().strong());
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(RichText::new(format!("{} Adapters Detected", monitor.networks.len())).color(TEXT_MUTED).size(11.0).monospace());
            });
        });
        ui.add_space(6.0);

        ScrollArea::vertical()
            .max_height(240.0)
            .show(ui, |ui| {
                egui::Grid::new("networks_detail_grid")
                    .striped(true)
                    .min_col_width(80.0)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("INTERFACE").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("RX RATE").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("TX RATE").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("TOTAL RECEIVED").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("TOTAL TRANSMITTED").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.end_row();

                        for net in &monitor.networks {
                            ui.label(RichText::new(&net.interface_name).color(TEXT_MAIN).strong().monospace());
                            ui.label(RichText::new(format!("↓ {:.1} KB/s", net.rx_rate_kbps)).color(STATUS_OK).strong().monospace());
                            ui.label(RichText::new(format!("↑ {:.1} KB/s", net.tx_rate_kbps)).color(ACCENT_TEAL).strong().monospace());
                            ui.label(RichText::new(format_bytes(net.rx_bytes)).color(TEXT_MUTED).monospace());
                            ui.label(RichText::new(format_bytes(net.tx_bytes)).color(TEXT_MUTED).monospace());
                            ui.end_row();
                        }
                    });
            });
    });
}
