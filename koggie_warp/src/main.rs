use warp::{self, Filter, Future};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

mod md2html;

/// A newtype to enforce our maximum allowed seconds.
struct Seconds(u64);

impl FromStr for Seconds {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        src.parse::<u64>().map_err(|_| ()).and_then(|num| {
            if num <= 5 {
                Ok(Seconds(num))
            } else {
                Err(())
            }
        })
    }
}

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

    // GET /listen
    let listen = warp::path("listen")
        .and(warp::path::param())
        // and_then create a `Future` that will simply wait N seconds...
        .and_then(|Seconds(seconds)| {
            Delay::new(Instant::now() + Duration::from_secs(seconds))
                // return the number of seconds again...
                .map(move |()| seconds)
                // An error from `Delay` means a big problem with the server...
                .map_err(|timer_err| {
                    eprintln!("timer error: {}", timer_err);
                    warp::reject::custom(timer_err)
                })
        })
        .map(|seconds| format!("I waited {} seconds!", seconds));


    // GET 404 error
    let e404 = warp::any()
        .map(|| {
            warp::reply::html(md2html::gen_page("../site/404.md"))
        });

    let routes = warp::get2().and(home
        .or(about)
        .or(artists)
        .or(calendar)
        .or(shows)
        .or(koggie)
        .or(robots)
        .or(res)
        .or(listen)
        .or(e404));

    warp::serve(routes)
        .run((std::net::Ipv4Addr::from_str(&args[1]).unwrap(), 8080));
}
