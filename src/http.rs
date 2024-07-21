use actix_web::HttpRequest;
use actix_web::{get, middleware::Compress, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use std::fs::{self};
use std::path::PathBuf;

use crate::color::{self, Color};
static MAIN_PAGE: &str = include_str!("pages/main.html");

#[get("/")]
async fn home() -> impl Responder {
    let no_html = env::var("NO_HTML").unwrap_or("".to_string());
    if no_html == "true" {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(MAIN_PAGE)
    } else {
        HttpResponse::Ok().body("Access denied")
    }
}
async fn handle_request(req: HttpRequest) -> Result<HttpResponse, std::io::Error> {
    let original_url = req.uri().to_string();
    let current_dir = env::current_dir()?;
    let mods_directories = fs::read_dir(current_dir.join("mods")).unwrap();

    let mut _check = false;

    for entry in mods_directories {
        if let Ok(entry) = entry {
            let folder = entry.file_name();
            let paths = vec!["http-client-files", "http-client-files-no-client-cache"];
            let file_parts: Vec<_> = original_url.split("/").skip(1).collect();
            let file_locale = file_parts.join("\\");
            for local in paths {
                let no_cache = env::var("NOCLIENTCACHE").unwrap_or("".to_string());
                if no_cache == "false" {
                    continue;
                };

                let path_buf = PathBuf::new()
                    .join(".")
                    .join("mods")
                    .join(folder.clone())
                    .join("resource-cache")
                    .join(local)
                    .join(file_locale.clone());
                println!("Pathbuf: {:?}", path_buf);

                if path_buf.exists() {
                    _check = true;
                    let file: actix_files::NamedFile =
                        actix_files::NamedFile::open_async(&path_buf).await.unwrap();
                    return Ok(file.into_response(&req));
                }
            }
        }
    }

    if !_check {
        return Ok(HttpResponse::NotFound().body("Resource not found"));
    }

    Ok(HttpResponse::NotFound().body("Not Found"))
}

pub async fn server() -> std::io::Result<()> {
    let port: String = env::var("ACCELERATOR_PORT").unwrap();
    println!("Starting accelerator in port {}",color::color(Color::Yellow, &port));
    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .service(home)
            .route("/{tail:.*}", web::get().to(handle_request))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
