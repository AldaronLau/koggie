use warp::{self, Filter};
use std::str::FromStr;

mod md2html;

fn main() {
    println!("Starting 'KOGGIE v{}'……", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 2);

    // GET /
    let home = warp::path::end()
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/home.md"))
        });

    // GET /about 
    let about = warp::path("about")
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/about.md"))
        });

    // GET /artists 
    let artists = warp::path("artists")
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/artists.md"))
        });


    // GET /calendar 
    let calendar = warp::path("calendar")
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/calendar.md"))
        });

    // GET /shows 
    let shows = warp::path("shows")
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/shows.md"))
        });

    // GET /koggie 
    let koggie = warp::path("koggie")
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/koggie.md"))
        });

    // GET /robots.txt 
    let robots = warp::path("robots.txt")
        .map(|| "User-agent: *\nDisallow: /res/");

    // GET /res/
    let res = warp::path("res")
        .and(warp::fs::dir("../res"));

    // GET 404 error
    let e404 = warp::any()
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/404.md"))
        });

    let routes = home
        .or(about)
        .or(artists)
        .or(calendar)
        .or(shows)
        .or(koggie)
        .or(robots)
        .or(res)
        .or(e404);

    warp::serve(routes)
        .run((std::net::Ipv4Addr::from_str(&args[1]).unwrap(), 8080));
}
