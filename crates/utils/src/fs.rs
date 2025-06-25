use std::path::Path;

pub fn resolve_datadir(datadir: &str) -> String {
    const BASE: &str = ".";
    Path::new(BASE).join(datadir).to_string_lossy().into_owned()
}
