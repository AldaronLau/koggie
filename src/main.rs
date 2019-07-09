use actix_web::{HttpServer, App as ActixApp, HttpRequest, HttpResponse, Responder, http::Method, web, guard};
use actix_files as fs;
use listenfd::ListenFd;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use pulldown_cmark::{Parser, Options, html};

const PAGE_TITLE: &str = "KAUG Radio & Concert Promotion";

fn load<'a>(html: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

fn gen_page(markdown_input: &str) -> String {
    let parser = Parser::new_ext(markdown_input, Options::all());
     
    let mut page = r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>"#.to_string();
    if let Some(title) = Some("TODO") {
        page.push_str(title);
        page.push_str(" | ");
    }
    page.push_str(PAGE_TITLE);
    page.push_str("</title>");
    page.push_str(concat!(
        include_str!("template/head.html"),
        include_str!("template/menu.html")
    ));
    html::push_html(&mut page, parser);
    page.push_str(include_str!("template/foot.html"));

    page
}

fn main() {
    println!("Starting 'KOGGIE v{}'……", env!("CARGO_PKG_VERSION"));

    let ip = std::fs::read_to_string("/usr/share/koggie/ip.txt").unwrap_or_else(
        |_e| {
            eprintln!("Failed to load file!");
            std::process::exit(1);
        }
    );

    println!("Running on IP {}", ip);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move ||
        ActixApp::new()
            .service(web::resource("/").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/home.md")) })))
            .service(web::resource("/about").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/about.md")) })))
            .service(web::resource("/artists").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/artists.md")) })))
            .service(web::resource("/calendar").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/calendar.md")) })))
            .service(web::resource("/shows").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/shows.md")) })))
            .service(web::resource("/koggie").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/koggie.md")) })))
            .service(web::resource("/robots.txt").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/koggie.md")) })))
            .service(web::resource("/robots.txt").route(
                web::get().to(|_req: HttpRequest| {
                    HttpResponse::Ok()
                        .content_type("text/plain")
                        .body("User-agent: *\nDisallow: /res/")
                }
            )))
            .service(fs::Files::new("/res", "/usr/share/koggie/public").show_files_listing())
            .default_service(
                web::resource("")
                    .route(web::get().to(|_req: HttpRequest| { load(gen_page("../site/404.md")) }))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                ),
            )
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(format!("10.0.0.159:8080")).unwrap()
    };

    server.run().unwrap();
}
