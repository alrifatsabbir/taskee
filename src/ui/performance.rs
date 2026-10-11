use crate::monitor::SystemMonitor;
use crate::ui::theme::*;
use egui::{Align, Layout, ProgressBar, RichText, ScrollArea, Ui};
use egui_plot::{Line, Plot, PlotPoints};

pub fn render(ui: &mut Ui, monitor: &SystemMonitor) {
    ui.add_space(4.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ CPU_TIME_SERIES ]").color(ACCENT_BLUE).size(12.0).monospace().strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{:.1}%", monitor.cpu_usage))
                                .color(ACCENT_BLUE)
                                .size(14.0)
                                .monospace()
                                .strong(),
                        );
                    });
                });
                ui.add_space(6.0);

                let points: PlotPoints = monitor.cpu_history.iter().copied().collect();
                let line = Line::new("CPU %", points)
                    .color(ACCENT_BLUE)
                    .width(2.0);

                Plot::new("cpu_plot_chart")
                    .height(240.0)
                    .include_y(0.0)
                    .include_y(100.0)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .show_axes([false, true])
                    .show_grid([true, true])
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
        });

        columns[1].vertical(|ui| {
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("[ RAM_TIME_SERIES ]").color(STATUS_OK).size(12.0).monospace().strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("{:.1}%", monitor.memory_usage_pct))
                                .color(STATUS_OK)
                                .size(14.0)
                                .monospace()
                                .strong(),
                        );
                    });
                });
                ui.add_space(6.0);

                let points: PlotPoints = monitor.memory_history.iter().copied().collect();
                let line = Line::new("RAM %", points)
                    .color(STATUS_OK)
                    .width(2.0);

                Plot::new("ram_plot_chart")
                    .height(240.0)
                    .include_y(0.0)
                    .include_y(100.0)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .show_axes([false, true])
                    .show_grid([true, true])
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
        });
    });

    ui.add_space(8.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("[ PER_CORE_UTILIZATION ]").color(ACCENT_BLUE).size(12.0).monospace().strong());
            ui.label(
                RichText::new(format!("({} Logical Threads)", monitor.cores.len()))
                    .color(TEXT_MUTED)
                    .size(11.0)
                    .monospace(),
            );
        });
        ui.add_space(6.0);

        ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                let columns_count = 4.min(monitor.cores.len().max(1));
                ui.columns(columns_count, |cols| {
                    for (idx, core) in monitor.cores.iter().enumerate() {
                        let target_col = idx % columns_count;
                        cols[target_col].vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&core.name).color(TEXT_MAIN).size(11.0).monospace());
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    let color = if core.usage > 85.0 {
                                        ACCENT_RED
                                    } else if core.usage > 60.0 {
                                        ACCENT_AMBER
                                    } else {
                                        ACCENT_BLUE
                                    };
                                    ui.label(
                                        RichText::new(format!("{:.0}%", core.usage))
                                            .color(color)
                                            .size(11.0)
                                            .monospace()
                                            .strong(),
                                    );
                                });
                            });

                            let prog = (core.usage / 100.0).clamp(0.0, 1.0);
                            let bar_color = if core.usage > 85.0 {
                                ACCENT_RED
                            } else if core.usage > 60.0 {
                                ACCENT_AMBER
                            } else {
                                ACCENT_BLUE
                            };
                            ui.add(ProgressBar::new(prog).fill(bar_color).animate(false));
                            ui.add_space(4.0);
                        });
                    }
                });
            });
    });
}
