use macro_ron::v2::Onset;

pub(crate) fn initial_surface(running_surface: &str) -> String {
    let Some(first) = running_surface.chars().next() else {
        return String::new();
    };
    first
        .to_uppercase()
        .chain(running_surface.chars().skip(1))
        .collect()
}

pub(crate) fn surface_onset(surface: &str) -> Option<Onset> {
    macro_ron::v2::normalize_surface_onset(surface, None)
}
