#![allow(dead_code)]

use egui::{Color32, CornerRadius, Frame, Margin, Stroke};

pub const BG_MAIN: Color32 = Color32::from_rgb(0x0D, 0x0E, 0x10);
pub const BG_CARD: Color32 = Color32::from_rgb(0x12, 0x13, 0x16);
pub const BG_INPUT: Color32 = Color32::from_rgb(0x18, 0x1A, 0x1F);
pub const BORDER: Color32 = Color32::from_rgb(0x22, 0x25, 0x2C);

pub const TEXT_MAIN: Color32 = Color32::from_rgb(0xEE, 0xEE, 0xEE);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x88, 0x8C, 0x96);
pub const STATUS_OK: Color32 = Color32::from_rgb(0x10, 0xB9, 0x81);

pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x25, 0x63, 0xEB);
pub const ACCENT_RED: Color32 = Color32::from_rgb(0xEF, 0x44, 0x44);
pub const ACCENT_PURPLE: Color32 = Color32::from_rgb(0x8B, 0x5C, 0xF6);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xF5, 0x9E, 0x0B);
pub const ACCENT_TEAL: Color32 = Color32::from_rgb(0x14, 0xB8, 0xA6);
pub const ACCENT_PINK: Color32 = Color32::from_rgb(0xEC, 0x48, 0x99);
pub const ACCENT_LIME: Color32 = Color32::from_rgb(0x84, 0xCC, 0x16);

pub fn card_frame() -> Frame {
    Frame {
        inner_margin: Margin::same(12),
        corner_radius: CornerRadius::same(6),
        fill: BG_CARD,
        stroke: Stroke::new(1.0, BORDER),
        ..Default::default()
    }
}

pub fn input_frame() -> Frame {
    Frame {
        inner_margin: Margin::symmetric(10, 6),
        corner_radius: CornerRadius::same(4),
        fill: BG_INPUT,
        stroke: Stroke::new(1.0, BORDER),
        ..Default::default()
    }
}

pub fn custom_card(inner: i8, radius: u8, fill: Color32, border_color: Color32) -> Frame {
    Frame {
        inner_margin: Margin::same(inner),
        corner_radius: CornerRadius::same(radius),
        fill,
        stroke: Stroke::new(1.0, border_color),
        ..Default::default()
    }
}
