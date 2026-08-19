pub(crate) fn initial_surface(running_surface: &str) -> String {
    let Some(first) = running_surface.chars().next() else {
        return String::new();
    };
    first
        .to_uppercase()
        .chain(running_surface.chars().skip(1))
        .collect()
}
