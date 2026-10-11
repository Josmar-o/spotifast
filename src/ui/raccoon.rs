//! The bundled Raccoon Hollow shell. Pages and controls remain native views.

use egui::{Align, Frame, Layout, Margin, Rect, Vec2, pos2, vec2};

use crate::app::App;
use crate::theme;

pub fn header(app: &mut App, ui: &mut egui::Ui) {
    let palette = app.palette;
    egui::Panel::top("raccoon-header")
        .exact_size(theme::TOP_BAR_HEIGHT + theme::titlebar_inset(ui.ctx()) + 16.0)
        .resizable(false)
        .frame(
            Frame::new()
                .fill(palette.panel)
                .inner_margin(Margin::symmetric(16, 8)),
        )
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    vec2(164.0, theme::TOP_BAR_HEIGHT),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        let (rect, _) =
                            ui.allocate_exact_size(Vec2::splat(34.0), egui::Sense::hover());
                        // The brand stays at the top even when the neighbouring toolbar grows.
                        let rect =
                            rect.translate(vec2(0.0, ui.min_rect().top() + 11.0 - rect.top()));
                        mascot(ui, rect);
                        ui.vertical(|ui| {
                            theme::text(ui, "SPOTIFAST", theme::bold(17.0), palette.text);
                            theme::text(
                                ui,
                                "Raccoon Hollow",
                                theme::regular(12.0),
                                palette.secondary,
                            );
                        });
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), theme::TOP_BAR_HEIGHT),
                    Layout::top_down(Align::Min),
                    |ui| super::topbar::show(app, ui),
                );
            });
        });
}

/// A tiny, original sprite for the shell, independent of Spotify cover art.
pub fn mascot(ui: &egui::Ui, rect: Rect) {
    const SPRITE: [&str; 16] = [
        "..dd......dd.......",
        ".dssd....dssd......",
        ".dssssddssssd......",
        "..ssssssssss.......",
        ".ssdddddddddss.....",
        ".sdwcddddwcds......",
        "..sdddddddss.......",
        "...sscdcsss........",
        "....ssssss.........",
        "...dssssssd........",
        "..dssssssssd...dd..",
        "..dssccccssd..dssd.",
        "..dssccccssddddssd.",
        "..dssccccssddssdd..",
        "...ddsssssdddssdd..",
        "..dddd..dddd.ddd...",
    ];
    let palette = theme::Palette::raccoon_hollow();
    let pixel = (rect.width().min(rect.height()) / 20.0).floor().max(1.0);
    let start = rect.center() - vec2(19.0, 16.0) * pixel / 2.0;
    for (y, row) in SPRITE.iter().enumerate() {
        for (x, character) in row.bytes().enumerate() {
            let color = match character {
                b'd' => palette.window,
                b's' => palette.secondary,
                b'c' | b'w' => palette.text,
                _ => continue,
            };
            ui.painter().rect_filled(
                Rect::from_min_size(
                    pos2(start.x + x as f32 * pixel, start.y + y as f32 * pixel),
                    Vec2::splat(pixel),
                ),
                0.0,
                color,
            );
        }
    }
}

/// Small bitmap controls keep the main transport and navigation in the same style.
pub fn pixel_icon(ui: &egui::Ui, icon: theme::Icon, rect: Rect, color: egui::Color32) -> bool {
    use theme::Icon;
    let rows = match icon {
        Icon::Play | Icon::PlayFilled | Icon::CirclePlay => {
            [0x40, 0x60, 0x70, 0x78, 0x78, 0x70, 0x60, 0x40]
        }
        Icon::Pause | Icon::PauseFilled => [0x66; 8],
        Icon::SkipForward | Icon::SkipForwardFilled => {
            [0x41, 0x61, 0x71, 0x79, 0x79, 0x71, 0x61, 0x41]
        }
        Icon::SkipBack | Icon::SkipBackFilled => [0x82, 0x86, 0x8e, 0x9e, 0x9e, 0x8e, 0x86, 0x82],
        Icon::House => [0x18, 0x3c, 0x7e, 0xff, 0x42, 0x5a, 0x5a, 0x7e],
        Icon::Search => [0x78, 0x84, 0x84, 0x84, 0x78, 0x0c, 0x06, 0x03],
        Icon::Library => [0xa4, 0xa4, 0xa4, 0xa2, 0xa2, 0xa2, 0xa1, 0xa1],
        Icon::HeartFilled => [0x66, 0xff, 0xff, 0xff, 0x7e, 0x3c, 0x18, 0x00],
        Icon::Heart => [0x66, 0x99, 0x81, 0x81, 0x42, 0x24, 0x18, 0x00],
        Icon::ChevronLeft | Icon::ArrowLeft => [0x08, 0x18, 0x30, 0x60, 0x60, 0x30, 0x18, 0x08],
        Icon::ChevronRight | Icon::ArrowRight => [0x10, 0x18, 0x0c, 0x06, 0x06, 0x0c, 0x18, 0x10],
        Icon::Mic => [0x18, 0x3c, 0x3c, 0x3c, 0x5a, 0x42, 0x3c, 0x18],
        Icon::X => [0x81, 0x42, 0x24, 0x18, 0x18, 0x24, 0x42, 0x81],
        Icon::Plus => [0x18, 0x18, 0x18, 0xff, 0xff, 0x18, 0x18, 0x18],
        Icon::Minus => [0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00],
        Icon::ListMusic => [0xf8, 0x00, 0xf8, 0x00, 0xf9, 0x03, 0xf5, 0x06],
        Icon::Settings => [0x18, 0x5a, 0x7e, 0xe7, 0xe7, 0x7e, 0x5a, 0x18],
        Icon::Volume | Icon::Volume1 | Icon::Volume2 => {
            [0x10, 0x32, 0x71, 0xf1, 0xf1, 0x71, 0x32, 0x10]
        }
        Icon::VolumeX => [0x10, 0x30, 0x75, 0xf2, 0xf2, 0x75, 0x30, 0x10],
        Icon::Repeat => [0x06, 0x7f, 0x82, 0x80, 0x01, 0x41, 0xfe, 0x60],
        Icon::Repeat1 => [0x06, 0x7f, 0x9a, 0x88, 0x09, 0x59, 0xfe, 0x60],
        Icon::Shuffle => [0x03, 0xc6, 0x2c, 0x18, 0x18, 0x2c, 0xc6, 0x03],
        _ => return false,
    };
    if rect.width().min(rect.height()) < 8.0 {
        return false;
    }
    let pixel = (rect.width().min(rect.height()) / 8.0).floor();
    let start = (rect.center() - Vec2::splat(4.0 * pixel)).round();
    for (y, row) in rows.iter().enumerate() {
        for x in 0..8 {
            if row & (0x80 >> x) != 0 {
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        start + vec2(x as f32 * pixel, y as f32 * pixel),
                        Vec2::splat(pixel),
                    ),
                    0.0,
                    color,
                );
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pixel_controls_fit_their_reserved_boxes() {
        let ctx = egui::Context::default();
        let rect = Rect::from_min_size(pos2(20.0, 20.0), Vec2::splat(24.0));
        for icon in [
            theme::Icon::PlayFilled,
            theme::Icon::PauseFilled,
            theme::Icon::Heart,
            theme::Icon::HeartFilled,
            theme::Icon::Search,
            theme::Icon::Repeat1,
        ] {
            let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
                assert!(pixel_icon(ui, icon, rect, egui::Color32::WHITE));
            });
            let mut count = 0;
            for shape in &output.shapes {
                if let egui::Shape::Rect(pixel) = &shape.shape {
                    assert!(rect.contains_rect(pixel.rect));
                    assert_eq!(pixel.corner_radius, egui::CornerRadius::ZERO);
                    count += 1;
                }
            }
            assert!(count > 0);
            output.textures_delta.clear();
        }
    }

    #[test]
    fn the_mascot_stays_inside_its_reserved_space() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            let rect = Rect::from_min_size(pos2(20.0, 20.0), Vec2::splat(40.0));
            mascot(ui, rect);
        });
        for shape in &output.shapes {
            if let egui::Shape::Rect(rect) = &shape.shape {
                assert!(rect.rect.min.x >= 20.0 && rect.rect.min.y >= 20.0);
                assert!(rect.rect.max.x <= 60.0 && rect.rect.max.y <= 60.0);
            }
        }
        output.textures_delta.clear();
    }
}
