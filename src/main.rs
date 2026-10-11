mod app;
mod monitor;
mod ui;

use app::TaskeeApp;
use eframe::egui::{self, CornerRadius, Stroke, Visuals};
use ui::theme::*;

fn generate_app_icon() -> egui::IconData {
    let width = 32;
    let height = 32;
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let dx = (x as f32 - 15.5) / 14.0;
            let dy = (y as f32 - 15.5) / 14.0;
            let dist = (dx * dx + dy * dy).sqrt();

            let is_hex_edge = (dx.abs() * 0.866 + dy.abs() * 0.5 <= 1.0)
                && (dy.abs() <= 1.0)
                && (dx.abs() * 0.866 + dy.abs() * 0.5 >= 0.72 || dy.abs() >= 0.72);

            let is_t_bar = (y >= 8 && y <= 11) && (x >= 9 && x <= 22);
            let is_t_stem = (y >= 11 && y <= 23) && (x >= 14 && x <= 17);

            if is_hex_edge {
                rgba.extend_from_slice(&[0x25, 0x63, 0xEB, 0xFF]);
            } else if is_t_bar || is_t_stem {
                rgba.extend_from_slice(&[0x10, 0xB9, 0x81, 0xFF]);
            } else if dist < 0.95 {
                rgba.extend_from_slice(&[0x12, 0x13, 0x16, 0xFF]);
            } else {
                rgba.extend_from_slice(&[0x0D, 0x0E, 0x10, 0x00]);
            }
        }
    }

    egui::IconData {
        rgba,
        width,
        height,
    }
}

fn configure_theme(ctx: &egui::Context) {
    let mut visuals = Visuals::dark();
    visuals.window_fill = BG_MAIN;
    visuals.panel_fill = BG_MAIN;
    visuals.faint_bg_color = BG_CARD;
    visuals.extreme_bg_color = BG_INPUT;
    visuals.code_bg_color = BG_CARD;

    visuals.window_corner_radius = CornerRadius::same(6);
    visuals.menu_corner_radius = CornerRadius::same(6);
    visuals.widgets.noninteractive.corner_radius = CornerRadius::same(4);
    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
    visuals.widgets.active.corner_radius = CornerRadius::same(4);
    visuals.widgets.open.corner_radius = CornerRadius::same(4);

    visuals.selection.bg_fill = ACCENT_BLUE;
    visuals.selection.stroke = Stroke::new(1.0, ACCENT_BLUE);

    visuals.widgets.noninteractive.bg_fill = BG_CARD;
    visuals.widgets.inactive.bg_fill = BG_CARD;
    visuals.widgets.hovered.bg_fill = BG_INPUT;
    visuals.widgets.active.bg_fill = ACCENT_BLUE;

    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT_BLUE);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_BLUE);

    ctx.set_visuals(visuals);
}

fn main() -> eframe::Result<()> {
    if std::env::var("WSL_DISTRO_NAME").is_ok() || std::env::var("WSL_INTEROP").is_ok() {
        unsafe {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
            std::env::remove_var("WAYLAND_DISPLAY");
        }
    }

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([880.0, 580.0])
            .with_title("Taskee — Desktop System Resource Monitor")
            .with_decorations(false)
            .with_icon(generate_app_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Taskee — Desktop System Resource Monitor",
        native_options,
        Box::new(|cc| {
            configure_theme(&cc.egui_ctx);
            Ok(Box::new(TaskeeApp::new(cc)))
        }),
    )
}
