use std::ffi::OsStr;
use std::iter::Iterator;
use std::path::Path;

pub const HEADER: &str = r#"<!DOCTYPE html>
<html lang="en">

  <head>
    <title>Dylan J. Wolff</title>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/kimeiga/bahunya/dist/bahunya.min.css">
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
    format!(r#"
    <nav>
            <a href="/">Home</a>
            {}
            <a href="https://github.com/dylanjwolff">GitHub</a>
            <a href="https://www.linkedin.com/in/dylan-j-wolff">LinkedIn</a>
    </nav>"#, inner)
}

pub fn render(nav: &str, body: &str) -> String {
    format!(
        r##"
    <main>
        <body>
            {}
            {}
        </body>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <br>
        <footer>
            <a href="#top">Back to top of page</a>
        </footer>
   </main>"##,
        nav, body
    )
}

pub const FOOTER: &str = r#"
 </html>
    "#;
