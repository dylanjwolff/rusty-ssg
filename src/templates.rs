use std::ffi::OsStr;
use std::iter::Iterator;
use std::path::Path;

pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://unpkg.com/bamboo.css/dist/dark.min.css">
  </head>

"#;
// <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.classless.min.css">
// <link rel="stylesheet" href="https://unpkg.com/mvp.css">

pub fn render_nav<'a>(items: impl Iterator<Item = (&'a Path, &'a OsStr)>) -> String {
    let inner = items
        .filter(|(_, stem)| stem.to_string_lossy() != "index")
        .map(|(path, stem)| {
            format!(
                r#"<a href="{}">{}</a>"#,
                path.with_extension("html").display(),
                stem.to_string_lossy()
            )
        })
        .collect::<Vec<_>>()
        .join("\n\t");
    format!("\n<nav>\n\t<a href=\"/\">Home</a>\n\t{}\n</nav>", inner)
}

pub fn render(nav: &str, body: &str) -> String {
    format!(
        r#"  <main>
        <body>
            {}
            <br />
            {}
          </body>
          </main>"#,
        nav, body
    )
}

pub const FOOTER: &str = r#"
 </html>
    "#;
