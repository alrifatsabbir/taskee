use crate::monitor::SystemMonitor;
use crate::ui::processes::ProcessSortBy;
use crate::ui::theme::*;
use crate::ui::{overview, performance, processes, storage_net};
use eframe::egui::{
    self, viewport::ResizeDirection, Align, Button, CornerRadius, CursorIcon, Frame, Layout,
    Margin, Pos2, Rect, RichText, Stroke, Vec2, ViewportCommand,
};
use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Overview,
    Performance,
    Processes,
    StorageNet,
}

pub struct TaskeeApp {
    monitor: SystemMonitor,
    active_tab: Tab,
    last_update: Instant,
    search_filter: String,
    sort_by: ProcessSortBy,
    status_message: Option<(String, bool)>,
    is_maximized: bool,
}

impl TaskeeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            monitor: SystemMonitor::new(),
            active_tab: Tab::Overview,
            last_update: Instant::now(),
            search_filter: String::new(),
            sort_by: ProcessSortBy::Cpu,
            status_message: None,
            is_maximized: false,
        }
    }
}

fn draw_cyber_badge(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(22.0, 22.0), egui::Sense::hover());
    let painter = ui.painter();
    let center = rect.center();
    let r = 9.5;

    let points: Vec<Pos2> = (0..6)
        .map(|i| {
            let angle = (i as f32) * std::f32::consts::PI / 3.0 - std::f32::consts::PI / 6.0;
            Pos2::new(center.x + r * angle.cos(), center.y + r * angle.sin())
        })
        .collect();

    painter.add(egui::Shape::closed_line(
        points,
        Stroke::new(1.5, ACCENT_BLUE),
    ));

    painter.line_segment(
        [
            Pos2::new(center.x - 4.5, center.y - 2.5),
            Pos2::new(center.x + 4.5, center.y - 2.5),
        ],
        Stroke::new(1.8, STATUS_OK),
    );
    painter.line_segment(
        [
            Pos2::new(center.x, center.y - 2.5),
            Pos2::new(center.x, center.y + 5.0),
        ],
        Stroke::new(1.8, STATUS_OK),
    );
}

fn handle_window_edge_resize(ctx: &egui::Context) {
    let screen_rect = ctx.viewport_rect();
    let margin = 6.0;

    let Some(pointer_pos) = ctx.input(|i| i.pointer.latest_pos()) else {
        return;
    };

    let on_left = pointer_pos.x <= screen_rect.left() + margin;
    let on_right = pointer_pos.x >= screen_rect.right() - margin;
    let on_top = pointer_pos.y <= screen_rect.top() + margin;
    let on_bottom = pointer_pos.y >= screen_rect.bottom() - margin;

    let dir = match (on_left, on_right, on_top, on_bottom) {
        (true, false, true, false) => Some((ResizeDirection::NorthWest, CursorIcon::ResizeNorthWest)),
        (false, true, true, false) => Some((ResizeDirection::NorthEast, CursorIcon::ResizeNorthEast)),
        (true, false, false, true) => Some((ResizeDirection::SouthWest, CursorIcon::ResizeSouthWest)),
        (false, true, false, true) => Some((ResizeDirection::SouthEast, CursorIcon::ResizeSouthEast)),
        (true, false, false, false) => Some((ResizeDirection::West, CursorIcon::ResizeWest)),
        (false, true, false, false) => Some((ResizeDirection::East, CursorIcon::ResizeEast)),
        (false, false, true, false) => Some((ResizeDirection::North, CursorIcon::ResizeNorth)),
        (false, false, false, true) => Some((ResizeDirection::South, CursorIcon::ResizeSouth)),
        _ => None,
    };

    if let Some((resize_dir, cursor)) = dir {
        ctx.set_cursor_icon(cursor);
        if ctx.input(|i| i.pointer.any_pressed()) {
            ctx.send_viewport_cmd(ViewportCommand::BeginResize(resize_dir));
        }
    }
}

fn window_btn(ui: &mut egui::Ui, kind: &str, is_red: bool) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(26.0, 22.0), egui::Sense::click());
    let mut bg = BG_CARD;
    let mut border = BORDER;
    let mut icon_color = TEXT_MUTED;

    if resp.hovered() {
        if is_red {
            bg = ACCENT_RED;
            border = ACCENT_RED;
            icon_color = TEXT_MAIN;
        } else {
            bg = BG_INPUT;
            icon_color = TEXT_MAIN;
        }
    } else if is_red {
        border = ACCENT_RED;
        icon_color = ACCENT_RED;
    }

    let p = ui.painter();
    p.rect_filled(rect, CornerRadius::same(3), bg);
    p.line_segment([rect.left_top(), rect.right_top()], Stroke::new(1.0, border));
    p.line_segment([rect.right_top(), rect.right_bottom()], Stroke::new(1.0, border));
    p.line_segment([rect.right_bottom(), rect.left_bottom()], Stroke::new(1.0, border));
    p.line_segment([rect.left_bottom(), rect.left_top()], Stroke::new(1.0, border));

    let center = rect.center();
    match kind {
        "min" => {
            p.line_segment(
                [Pos2::new(center.x - 4.5, center.y), Pos2::new(center.x + 4.5, center.y)],
                Stroke::new(1.5, icon_color),
            );
        }
        "max" => {
            let box_rect = Rect::from_center_size(center, Vec2::new(8.0, 8.0));
            p.line_segment([box_rect.left_top(), box_rect.right_top()], Stroke::new(1.2, icon_color));
            p.line_segment([box_rect.right_top(), box_rect.right_bottom()], Stroke::new(1.2, icon_color));
            p.line_segment([box_rect.right_bottom(), box_rect.left_bottom()], Stroke::new(1.2, icon_color));
            p.line_segment([box_rect.left_bottom(), box_rect.left_top()], Stroke::new(1.2, icon_color));
        }
        "close" => {
            p.line_segment(
                [Pos2::new(center.x - 4.0, center.y - 4.0), Pos2::new(center.x + 4.0, center.y + 4.0)],
                Stroke::new(1.6, icon_color),
            );
            p.line_segment(
                [Pos2::new(center.x + 4.0, center.y - 4.0), Pos2::new(center.x - 4.0, center.y + 4.0)],
                Stroke::new(1.6, icon_color),
            );
        }
        _ => {}
    }
    resp
}

impl eframe::App for TaskeeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.last_update.elapsed() >= std::time::Duration::from_millis(1000) {
            self.monitor.refresh();
            self.last_update = Instant::now();
        }

        ui.ctx().request_repaint_after(std::time::Duration::from_millis(200));

        handle_window_edge_resize(ui.ctx());

        let title_bar_frame = Frame {
            inner_margin: Margin::symmetric(12, 6),
            fill: BG_MAIN,
            stroke: Stroke::new(1.0, BORDER),
            ..Default::default()
        };

        title_bar_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                draw_cyber_badge(ui);
                ui.add_space(6.0);

                ui.label(
                    RichText::new("TASKEE")
                        .color(TEXT_MAIN)
                        .size(15.0)
                        .strong()
                        .monospace(),
                );
                ui.label(
                    RichText::new("SYS-HUD")
                        .color(TEXT_MUTED)
                        .size(10.0)
                        .monospace(),
                );

                ui.add_space(20.0);

                let tabs = [
                    (Tab::Overview, "01 OVERVIEW"),
                    (Tab::Performance, "02 PERFORMANCE"),
                    (Tab::Processes, "03 PROCESSES"),
                    (Tab::StorageNet, "04 DISKS & NET"),
                ];

                for (tab, label) in tabs {
                    let is_active = self.active_tab == tab;
                    let bg = if is_active { BG_INPUT } else { BG_CARD };
                    let border = if is_active { ACCENT_BLUE } else { BORDER };
                    let text_color = if is_active { TEXT_MAIN } else { TEXT_MUTED };

                    let btn = Button::new(RichText::new(label).color(text_color).size(11.0).monospace().strong())
                        .fill(bg)
                        .corner_radius(CornerRadius::same(4))
                        .stroke(Stroke::new(1.0, border))
                        .min_size(Vec2::new(104.0, 24.0));

                    if ui.add(btn).clicked() {
                        self.active_tab = tab;
                    }
                }

                let remaining_width = ui.available_width() - 170.0;
                if remaining_width > 20.0 {
                    let (_, drag_resp) = ui.allocate_exact_size(
                        Vec2::new(remaining_width, 24.0),
                        egui::Sense::drag(),
                    );
                    if drag_resp.dragged() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
                    }
                    if drag_resp.double_clicked() {
                        self.is_maximized = !self.is_maximized;
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(self.is_maximized));
                    }
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if window_btn(ui, "close", true).clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                    }

                    if window_btn(ui, "max", false).clicked() {
                        self.is_maximized = !self.is_maximized;
                        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(self.is_maximized));
                    }

                    if window_btn(ui, "min", false).clicked() {
                        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
                    }

                    ui.add_space(8.0);
                    ui.label(RichText::new("● LIVE").color(STATUS_OK).size(10.0).monospace().strong());
                });
            });
        });

        ui.add_space(4.0);

        let content_frame = Frame {
            inner_margin: Margin::symmetric(12, 8),
            fill: BG_MAIN,
            ..Default::default()
        };

        content_frame.show(ui, |ui| {
            match self.active_tab {
                Tab::Overview => overview::render(ui, &self.monitor),
                Tab::Performance => performance::render(ui, &self.monitor),
                Tab::Processes => processes::render(
                    ui,
                    &mut self.monitor,
                    &mut self.search_filter,
                    &mut self.sort_by,
                    &mut self.status_message,
                ),
                Tab::StorageNet => storage_net::render(ui, &self.monitor),
            }

            ui.with_layout(Layout::right_to_left(Align::BOTTOM), |ui| {
                let (grip_rect, grip_resp) = ui.allocate_exact_size(Vec2::new(12.0, 12.0), egui::Sense::hover());
                if grip_resp.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeSouthEast);
                }
                let painter = ui.painter();
                painter.line_segment(
                    [Pos2::new(grip_rect.right() - 8.0, grip_rect.bottom()), Pos2::new(grip_rect.right(), grip_rect.bottom() - 8.0)],
                    Stroke::new(1.0, BORDER),
                );
                painter.line_segment(
                    [Pos2::new(grip_rect.right() - 4.0, grip_rect.bottom()), Pos2::new(grip_rect.right(), grip_rect.bottom() - 4.0)],
                    Stroke::new(1.0, BORDER),
                );
            });
        });
    }
}
