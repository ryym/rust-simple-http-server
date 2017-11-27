use std::fs;
use std::path::Path;
use super::{AppResult, AppError};

pub fn generate(dir: &Path, cwd: &Path) -> AppResult<String> {
    let mut html = format!(r#"
    <!DOCTYPE html>
    <html lang="en">
    <title>Files in {}</title>
    <head><body>
    "#, &dir.to_string_lossy());

    match dir.strip_prefix(cwd) {
        Err(_) => Err(AppError::new("Failed to strip dir path prefix")),
        Ok(base) => {
            let base_name = make_base_name(base);
            html.push_str(&format!("<h1>Index of {}</h1><ul>", base_name));

            for entry in fs::read_dir(dir)? {
                let name = entry?.file_name().to_string_lossy().into_owned();
                let path = format!("{}{}", base_name, name);
                html.push_str(&format!(r#"<li><a href="{}">{}</a></li>"#, &path, &name));
                html.push('\n');
            }

            html.push_str("</ul></body></html>");
            Ok(html)
        }
    }
}

fn make_base_name(base: &Path) -> String {
    let mut base_name = "/".to_string() + &base.to_string_lossy();
    if base_name.len() > 1 {
        base_name.push('/');
    }
    base_name
}
