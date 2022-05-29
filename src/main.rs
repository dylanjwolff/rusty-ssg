mod templates;

static CONTENT_DIR: &str = "content";
static PUBLIC_DIR: &str = "public";
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::{fs, net::SocketAddr, path::PathBuf, thread, time::Duration, io};
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get_service, Router};
use tower_http::services::ServeDir;

// Serves files inside the `public` directory at `GET /public/*`

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), anyhow::Error> {
    rebuild();
    let (sender, receiver) = channel();
    let mut watcher = watcher(sender, Duration::from_millis(10)).unwrap();
    watcher
        .watch(CONTENT_DIR, RecursiveMode::Recursive)
        .expect("Directory watcher failed to initialize");

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(10));
        match receiver.recv() {
            Ok(event) => {
                println!("{:?}", event);
                rebuild()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    });

    let serve_dir_service =
        get_service(ServeDir::new("public")).handle_error(|error: io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });

    let app = Router::new().nest("/", serve_dir_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("serving site on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}


fn rebuild() {
    let _ = fs::remove_dir_all(PUBLIC_DIR);

    let files = walkdir::WalkDir::new(CONTENT_DIR)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().extension().map(|e| e == "md").is_some());

    files
        .map(|md_f| {
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
