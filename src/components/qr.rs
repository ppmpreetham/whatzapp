use std::sync::Arc;

use blenderui::prelude::*;
use qrcode::{Color, QrCode};

const SCALE: usize = 6;
const QUIET: usize = 4;

pub fn qr_pixels(data: &str) -> Option<(Vec<u8>, usize)> {
    let code = QrCode::new(data.as_bytes()).ok()?;
    let modules = code.width();
    let side = (modules + 2 * QUIET) * SCALE;

    let mut rgba = vec![255u8; side * side * 4];

    for (y, row) in code.to_colors().chunks_exact(modules).enumerate() {
        for (x, &color) in row.iter().enumerate() {
            if color == Color::Dark {
                for py in (y + QUIET) * SCALE..(y + QUIET + 1) * SCALE {
                    let start = (py * side + (x + QUIET) * SCALE) * 4;
                    for pixel in rgba[start..start + SCALE * 4].as_chunks_mut::<4>().0 {
                        pixel[..3].fill(0);
                    }
                }
            }
        }
    }

    Some((rgba, side))
}

pub fn qr_image(data: &str) -> Option<(Arc<Image>, f32)> {
    let (rgba, side) = qr_pixels(data)?;
    Some((Arc::new(Image::from_rgba(side as u32, side as u32, &rgba)?), side as f32))
}

#[component]
pub fn Qr(data: SharedString) {
    let (image, side) = qr_image(&data).expect("qr data");
    rsx! { {img().w(px(side)).h(px(side)).rounded_lg().image(image)} }
}
