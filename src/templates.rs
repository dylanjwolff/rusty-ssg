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

// Other "classless" css templates
// <link rel="stylesheet" href="https://unpkg.com/mvp.css">

pub fn render_nav<'a>(items: impl Iterator<Item = (&'a Path, &'a OsStr)>) -> String {
    let inner = items
        .filter(|(_, stem)| stem.to_string_lossy() != "index")
        .map(|(path, stem)| {
            format!(
                r#"<li><a href="{}">{}</a></li>"#,
                path.with_extension("html").display(),
                stem.to_string_lossy()
            )
        })
        .collect::<Vec<_>>()
        .join("\n\t");
    format!(r##"
    <nav><ul>
            <li><a href="/">Home</a></li>
            {}
            <li>
            <a href="#links">Links</a>
            <ul>
                <li><a href="https://github.com/dylanjwolff">GitHub</a></li>
                <li><a href="https://www.linkedin.com/in/dylan-j-wolff">LinkedIn</a></li>
            </ul>
            </li>
    </ul></nav>"##, inner)
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
   </main>"##,
        nav, body
    )
}

pub const FOOTER: &str = r##"

        <footer>
            <a href="#top">Back to top of page</a>
        </footer>
 </html>
    "##;
