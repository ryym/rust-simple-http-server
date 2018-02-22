pub fn content_type(ext: &str) -> Option<&str> {
    match ext {
        "html" => Some("text/html"),
        "js" => Some("text/javascript"),
        "css" => Some("text/css"),
        "json" => Some("application/json"),
        "png" => Some("image/png"),
        _ => None,
    }
}
