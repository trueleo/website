use std::{collections::HashMap, sync::OnceLock};

use axum::{
    extract::Path,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use comrak::Options;
use handlebars::Handlebars;
use static_files::Resource;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod web_files {
    include!(concat!(env!("OUT_DIR"), "/web.rs"));
}

mod markdown_files {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

fn web_static_files() -> &'static HashMap<&'static str, Resource> {
    static STATIC_FILES: OnceLock<HashMap<&'static str, Resource>> = std::sync::OnceLock::new();
    STATIC_FILES.get_or_init(web_files::generate)
}

fn markdown_static_files() -> &'static HashMap<&'static str, Resource> {
    static STATIC_FILES: OnceLock<HashMap<&'static str, Resource>> = std::sync::OnceLock::new();
    STATIC_FILES.get_or_init(markdown_files::generate)
}

async fn index() -> Html<&'static [u8]> {
    Html(
        web_static_files()
            .get("index.html")
            .expect("index.html is loaded")
            .data,
    )
}

#[derive(serde::Serialize)]
struct BlogTemplateValue {
    html_content: String,
}

async fn blog_md_handler(name: Path<String>) -> axum::response::Result<Html<String>> {
    // replace - in url name with _ for files.
    let name = name.trim().replace('-', "_") + ".md";

    let markdown = markdown_static_files()
        .get(name.as_str())
        .map(|resource| String::from_utf8_lossy(resource.data))
        .unwrap_or("# 404 File Not Found".into());
    let markdown_to_html = comrak::markdown_to_html(&markdown, &Options::default());

    let blog_page = web_static_files()
        .get("blog.html")
        .map(|x| String::from_utf8_lossy(x.data))
        .expect("blog.html is loaded");

    let templater = Handlebars::new();
    let res = templater
        .render_template(
            &blog_page,
            &BlogTemplateValue {
                html_content: markdown_to_html,
            },
        )
        .map_err(|x| x.to_string())?;

    Ok(Html(res))
}

async fn serve_from_static(uri: Uri) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path().trim_start_matches('/');
    let resource = web_static_files().get(path).ok_or(StatusCode::NOT_FOUND)?;
    Ok(([(header::CONTENT_TYPE, resource.mime_type)], resource.data))
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cors = CorsLayer::very_permissive();
    let app = Router::new()
        .route("/", get(index))
        .route("/blog/:name", get(blog_md_handler))
        .fallback(serve_from_static)
        .layer(cors);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
