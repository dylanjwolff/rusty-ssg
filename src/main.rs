mod templates;

static CONTENT_DIR: &str = "content";
static PUBLIC_DIR: &str = "public";
use axum::handler::Handler;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{convert::Infallible, fs, net::SocketAddr, path::Path, thread, time::Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    rebuild();
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_millis(10)).unwrap();
    watcher
        .watch(CONTENT_DIR, RecursiveMode::Recursive)
        .expect("Directory watcher failed to initialize");

    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(10));
        match receiver.recv() {
            Ok(event) => {
                println!("{:?}", event);
                rebuild()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    });

    let app = axum::Router::new().nest(
        "/",
        axum::routing::get(tower_http::services::fs::ServeDir::new(PUBLIC_DIR)).handle_error(
            |error: std::io::Error| {
                Ok::<_, Infallible>((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            },
        ),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

use std::path::PathBuf;

fn rebuild() {
    let _ = fs::remove_dir_all(PUBLIC_DIR);

    let files = walkdir::WalkDir::new(CONTENT_DIR)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().extension().map(|e| e == "md").is_some());

    files
        .map(|md_f| {
            println!("FILE");
            let markdown = fs::read_to_string(md_f.path()).unwrap();
            let parser = pulldown_cmark::Parser::new_ext(&markdown, pulldown_cmark::Options::all());
            let mut body = String::new();

            pulldown_cmark::html::push_html(&mut body, parser);
            let rendered = format!(
                "{} {} {}",
                templates::HEADER,
                templates::render_body(&body),
                templates::FOOTER
            );

            let mut html_path: PathBuf = [PUBLIC_DIR, md_f.file_name().to_str().unwrap()]
                .iter()
                .collect();

            html_path.set_extension("html");

            let _ = fs::create_dir_all(html_path.parent().unwrap());

            (html_path, rendered)
        })
        .for_each(|(path, htmls)| fs::write(path, htmls).unwrap());
}
