use crate::monitor::SystemMonitor;
use crate::ui::theme::*;
use egui::{
    Align, Button, Color32, CornerRadius, Layout, RichText, ScrollArea, Stroke, TextEdit, Ui,
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ProcessSortBy {
    Cpu,
    Memory,
    Pid,
    Name,
    Runtime,
}

pub fn render(
    ui: &mut Ui,
    monitor: &mut SystemMonitor,
    search_filter: &mut String,
    sort_by: &mut ProcessSortBy,
    status_message: &mut Option<(String, bool)>,
) {
    ui.add_space(4.0);

    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("FILTER:").color(TEXT_MUTED).size(11.0).monospace().strong());
            ui.add(
                TextEdit::singleline(search_filter)
                    .hint_text("Search name or PID...")
                    .desired_width(220.0),
            );

            if !search_filter.is_empty() && ui.button(RichText::new("Clear").size(11.0)).clicked() {
                search_filter.clear();
            }

            ui.label(RichText::new("|").color(BORDER));
            ui.label(RichText::new("SORT:").color(TEXT_MUTED).size(11.0).monospace().strong());

            let cpu_btn = Button::new(RichText::new("CPU").size(11.0).monospace())
                .fill(if *sort_by == ProcessSortBy::Cpu { ACCENT_BLUE } else { BG_INPUT })
                .corner_radius(CornerRadius::same(4))
                .stroke(Stroke::new(1.0, BORDER));
            if ui.add(cpu_btn).clicked() {
                *sort_by = ProcessSortBy::Cpu;
            }

            let mem_btn = Button::new(RichText::new("MEM").size(11.0).monospace())
                .fill(if *sort_by == ProcessSortBy::Memory { STATUS_OK } else { BG_INPUT })
                .corner_radius(CornerRadius::same(4))
                .stroke(Stroke::new(1.0, BORDER));
            if ui.add(mem_btn).clicked() {
                *sort_by = ProcessSortBy::Memory;
            }

            let pid_btn = Button::new(RichText::new("PID").size(11.0).monospace())
                .fill(if *sort_by == ProcessSortBy::Pid { ACCENT_PURPLE } else { BG_INPUT })
                .corner_radius(CornerRadius::same(4))
                .stroke(Stroke::new(1.0, BORDER));
            if ui.add(pid_btn).clicked() {
                *sort_by = ProcessSortBy::Pid;
            }

            let name_btn = Button::new(RichText::new("NAME").size(11.0).monospace())
                .fill(if *sort_by == ProcessSortBy::Name { ACCENT_AMBER } else { BG_INPUT })
                .corner_radius(CornerRadius::same(4))
                .stroke(Stroke::new(1.0, BORDER));
            if ui.add(name_btn).clicked() {
                *sort_by = ProcessSortBy::Name;
            }

            let time_btn = Button::new(RichText::new("RUNTIME").size(11.0).monospace())
                .fill(if *sort_by == ProcessSortBy::Runtime { ACCENT_TEAL } else { BG_INPUT })
                .corner_radius(CornerRadius::same(4))
                .stroke(Stroke::new(1.0, BORDER));
            if ui.add(time_btn).clicked() {
                *sort_by = ProcessSortBy::Runtime;
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(format!(
                        "TOTAL: {} | RUNNING: {} | SLEEP: {}",
                        monitor.processes.len(),
                        monitor.running_processes_count,
                        monitor.sleeping_processes_count
                    ))
                    .color(TEXT_MUTED)
                    .size(11.0)
                    .monospace(),
                );
            });
        });
    });

    if let Some((msg, success)) = status_message {
        ui.add_space(6.0);
        let border_color = if *success { STATUS_OK } else { ACCENT_RED };
        let fill_color = if *success {
            Color32::from_rgba_premultiplied(16, 185, 129, 25)
        } else {
            Color32::from_rgba_premultiplied(239, 68, 68, 25)
        };

        custom_card(8, 4, fill_color, border_color).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(msg.as_str()).color(border_color).monospace().strong());
            });
        });
    }

    ui.add_space(6.0);

    let mut process_to_kill: Option<u32> = None;

    let mut sorted_procs = monitor.processes.clone();
    match sort_by {
        ProcessSortBy::Cpu => sorted_procs.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
        ProcessSortBy::Memory => sorted_procs.sort_by(|a, b| b.memory_bytes.cmp(&a.memory_bytes)),
        ProcessSortBy::Pid => sorted_procs.sort_by(|a, b| a.pid.cmp(&b.pid)),
        ProcessSortBy::Name => sorted_procs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
        ProcessSortBy::Runtime => sorted_procs.sort_by(|a, b| b.run_time_secs.cmp(&a.run_time_secs)),
    }

    card_frame().show(ui, |ui| {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("workstation_process_grid")
                    .striped(true)
                    .min_col_width(70.0)
                    .spacing([18.0, 8.0])
                    .show(ui, |ui| {
                        ui.label(RichText::new("PID").strong().color(ACCENT_BLUE).monospace().size(11.0));
                        ui.label(RichText::new("STATUS").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("NAME").strong().color(TEXT_MAIN).monospace().size(11.0));
                        ui.label(RichText::new("CPU %").strong().color(TEXT_MAIN).monospace().size(11.0));
                        ui.label(RichText::new("RAM (MB)").strong().color(TEXT_MAIN).monospace().size(11.0));
                        ui.label(RichText::new("RAM %").strong().color(TEXT_MAIN).monospace().size(11.0));
                        ui.label(RichText::new("RUNTIME").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.label(RichText::new("ACTION").strong().color(TEXT_MUTED).monospace().size(11.0));
                        ui.end_row();

                        let filter_lower = search_filter.trim().to_lowercase();

                        for proc in &sorted_procs {
                            let pid_str = proc.pid.to_string();
                            if !filter_lower.is_empty()
                                && !proc.name.to_lowercase().contains(&filter_lower)
                                && !pid_str.contains(&filter_lower)
                            {
                                continue;
                            }

                            ui.label(RichText::new(&pid_str).color(TEXT_MUTED).monospace());

                            let is_running = proc.status.contains("Run");
                            let status_color = if is_running { STATUS_OK } else { TEXT_MUTED };
                            let status_text = if is_running { "RUN" } else { "SLEEP" };
                            ui.label(RichText::new(status_text).color(status_color).size(10.0).monospace().strong());

                            ui.label(RichText::new(&proc.name).color(TEXT_MAIN).strong().monospace());

                            let cpu_text = format!("{:.1}%", proc.cpu_usage);
                            let cpu_color = if proc.cpu_usage > 50.0 {
                                ACCENT_RED
                            } else if proc.cpu_usage > 15.0 {
                                ACCENT_AMBER
                            } else {
                                ACCENT_BLUE
                            };
                            ui.label(RichText::new(cpu_text).color(cpu_color).strong().monospace());

                            let mem_mb = proc.memory_bytes as f64 / (1024.0 * 1024.0);
                            ui.label(RichText::new(format!("{:.1}", mem_mb)).color(STATUS_OK).monospace());
                            ui.label(RichText::new(format!("{:.1}%", proc.memory_pct)).color(TEXT_MUTED).monospace());

                            let mins = proc.run_time_secs / 60;
                            let secs = proc.run_time_secs % 60;
                            ui.label(RichText::new(format!("{:02}:{:02}", mins, secs)).color(TEXT_MUTED).monospace());

                            let kill_btn = Button::new(
                                RichText::new("KILL")
                                    .color(ACCENT_RED)
                                    .size(10.0)
                                    .monospace()
                                    .strong(),
                            )
                            .fill(BG_INPUT)
                            .corner_radius(CornerRadius::same(4))
                            .stroke(Stroke::new(1.0, ACCENT_RED));

                            if ui.add(kill_btn).clicked() {
                                process_to_kill = Some(proc.pid);
                            }

                            ui.end_row();
                        }
                    });
            });
    });

    if let Some(pid) = process_to_kill {
        let killed = monitor.kill_process(pid);
        if killed {
            *status_message = Some((format!("SIGNAL SENT: Successfully terminated PID {}", pid), true));
        } else {
            *status_message = Some((format!("TERMINATION FAILED: Unable to kill PID {}", pid), false));
        }
    }
}
