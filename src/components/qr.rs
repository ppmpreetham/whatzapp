use blenderui::prelude::*;
use qrcode::{Color, QrCode};

const SCALE: usize = 6;
const QUIET: usize = 4;

#[component]
pub fn Qr(data: SharedString) {
    let code = QrCode::new(data.as_bytes()).expect("qr data");
    let modules = code.width();
    let side = (modules + 2 * QUIET) * SCALE;

    let mut rgba = vec![255u8; side * side * 4];

    for (y, row) in code.to_colors().chunks_exact(modules).enumerate() {
        for (x, &color) in row.iter().enumerate() {
            if color == Color::Dark {
                for py in (y + QUIET) * SCALE..(y + QUIET + 1) * SCALE {
                    let start = (py * side + (x + QUIET) * SCALE) * 4;
                    for pixel in rgba[start..start + SCALE * 4].chunks_exact_mut(4) {
                        pixel[..3].fill(0);
                    }
                }
            }
        }
    }

    rsx! {
        {img()
            .w(px(side as f32))
            .h(px(side as f32))
            .rounded_lg()
            .image_rgba(side as u32, side as u32, rgba)}
    }
}
