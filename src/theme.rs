fn key(name: &str) -> &str {
    match name {
        "background" => "region.background",
        "foreground" => "text",
        "border" => "box.border",
        "primary.background" => "button.background.selected",
        "primary.foreground" => "button.text.selected",
        "secondary.background" => "button.background",
        other => other,
    }
}

pub fn color(name: &str) -> String {
    match blenderui::theme_color(key(name)) {
        Some([r, g, b, a]) => {
            let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
            format!("#{:02x}{:02x}{:02x}{:02x}", byte(r), byte(g), byte(b), byte(a))
        }
        None => "#ff00ff".into(),
    }
}

pub fn vars() -> String {
    [
        ("bg", "background"),
        ("fg", "foreground"),
        ("muted", "muted.foreground"),
        ("border", "border"),
        ("surface", "secondary.background"),
        ("chip", "secondary.active.background"),
        ("popover", "secondary.background"),
        ("in", "bubble.in.background"),
        ("out", "bubble.out.background"),
        ("outfg", "bubble.out.foreground"),
        ("accent", "primary.background"),
        ("onaccent", "primary.foreground"),
    ]
    .iter()
    .map(|(var, name)| format!("--wz-{var}:{}", color(name)))
    .collect::<Vec<_>>()
    .join(";")
}

pub fn rgb(name: &str) -> [f32; 3] {
    blenderui::theme_color(key(name)).map_or([1.0, 0.0, 1.0], |[r, g, b, _]| [r, g, b])
}
