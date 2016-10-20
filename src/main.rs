extern crate iron;
extern crate router;
extern crate crowbook;
extern crate urlencoded;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::error::HttpResult;
use hyper::server::Listening;
use router::Router;
use std::error::Error;

fn main() {
    fn router() -> Router {
        let mut router = Router::new();
        router.get("/", show_en, "main");
        router.get("/en", show_en, "main_bis");
        router.get("/style.css", show_css, "style");
        router.get("/foundation.css", show_foundation_css, "foundation");
        router.get("/normalize.css", show_normalize_css, "normalize");
        router.get("/foundation.js", show_foundation_js, "foundation_js");
        // router.get("/fr", show_fr);
        // router.get("/doc_en", show_doc_en);
        // router.get("/doc_fr", show_doc_fr);
        // router.get("/style.css", show_css);
        // router.get("/serialize.js", show_serialize_js);
        // router.get("/main.js", show_main_js);
        // router.post("/result", show_result);
        // router.get("/foundation.css", show_foundation_css);
        // router.get("/normalize.css", show_normalize_css);
        // router.get("/foundation.js", show_foundation_js);
        // router.get("/caribon.png", show_logo);
        router
    }

    fn show_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/main.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }
    
    fn show_foundation_js(_: &mut Request) -> IronResult<Response> {
        let js = include_str!("html/foundation.min.js");
        let content_type = "text/javascript".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, js)))
    }

    fn show_foundation_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/foundation.min.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn show_normalize_css(_: &mut Request) -> IronResult<Response> {
        let css = include_str!("html/normalize.css");
        let content_type = "text/css".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, css)))
    }

    fn show_en(_: &mut Request) -> IronResult<Response> {
        let text = include_str!("html/en.html");
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, text)))
    }

    let ips = vec!("127.0.0.1:3000");
    let mut res:Vec<HttpResult<Listening>> = vec!();
    
    for ip in ips {
        res.push(Iron::new(router()).http(ip));
    }
}
