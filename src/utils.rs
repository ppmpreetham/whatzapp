use crate::theme;

/// join classes
pub fn cx(parts: &[&str]) -> String {
    parts.iter().copied().filter(|p| !p.is_empty()).collect::<Vec<_>>().join(" ")
}

/// theme colors inlinning
pub fn style(props: &[(&str, &str)]) -> String {
    props
        .iter()
        .map(|(prop, name)| format!("{prop}:{}", theme::color(name)))
        .collect::<Vec<_>>()
        .join(";")
}

/// initial or ?
pub fn initial(name: &str) -> String {
    name.chars().next().map_or('?'.to_string(), |c| c.to_uppercase().to_string())
}
