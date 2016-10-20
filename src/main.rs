extern crate iron;
extern crate router;
extern crate crowbook;
extern crate urlencoded;
extern crate hyper;
extern crate tempfile;

mod config;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::error::HttpResult;
use hyper::server::Listening;
use router::Router;
use crowbook::Book;
use crowbook::Number;
use crowbook::HtmlSingleRenderer;
use crowbook::EpubRenderer;
use crowbook::InfoLevel;
use tempfile::NamedTempFile;

use std::error::Error;
use std::io::Write;
use std::io::Read;

use config::Config;


fn main() {
    fn router() -> Router {
        let mut router = Router::new();
        router.get("/", show_en, "main");
        router.get("/en", show_en, "main_bis");
        router.get("/style.css", show_css, "style");
        router.get("/foundation.css", show_foundation_css, "foundation");
        router.get("/normalize.css", show_normalize_css, "normalize");
        router.get("/foundation.js", show_foundation_js, "foundation_js");
        router.get("/crowbook.png", show_logo, "logo");
        router.post("/result", show_result, "result");
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

    fn show_logo(_: &mut Request) -> IronResult<Response> {
        let img:&'static[u8] = include_bytes!("html/crowbook.png");
        let content_type = "image/png".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, img)))
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

    fn render_book(config: &Config) -> crowbook::Result<Response> {
        let mut tmpfile = NamedTempFile::new().unwrap();
        tmpfile.write_all(config.text.as_bytes()).unwrap();

        match config.output.as_str() {
            "html" => {
                let mut book = try!(Book::new_from_markdown_file(tmpfile.path().to_str().unwrap(), InfoLevel::Quiet, &[]));
                let mut renderer = HtmlSingleRenderer::new(&book);
                let content = try!(renderer.render_book());
                let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
                return Ok(Response::with((content_type, status::Ok, content)));
            },
            "epub" => {
                let mut epubfile = NamedTempFile::new().unwrap();
                let book = try!(Book::new_from_markdown_file(tmpfile.path().to_str().unwrap(),
                                                             InfoLevel::Quiet,
                                                             &[("output.epub", epubfile.path().to_str().unwrap())]));
                let mut renderer = EpubRenderer::new(&book);

                try!(renderer.render_book());
                let mut content = vec!();
                epubfile.read_to_end(&mut content).unwrap();
                let content_type = "application/epub+zip".parse::<Mime>().unwrap();
                return Ok(Response::with((content_type, status::Ok, content)));
            }
            _ => {
                return Err(crowbook::Error::default(crowbook::Source::empty(),
                                                    "Unrecognized output format"));
            },
        }
    }

    fn show_error<S: ::std::fmt::Display>(error: S) -> Response {
        let content_type = "text/html; charset=UTF-8".parse::<Mime>().unwrap();
        let content = format!("<html><body>{}</body></html>", error);
        Response::with((content_type, status::Ok, content))
    }

    fn show_result(request: &mut Request) -> IronResult<Response> {
        let result:Result<Config,String> = Config::new_from_request(request);
        let response = match result {
            Ok(config) => {
                match render_book(&config) {
                    Ok(response) => response,
                    Err(e) => show_error(format!("Error: {}", e)),
                }
            },
            Err(e) => show_error(format!("Error: {}", e)),
        };

        Ok(response)
    }
    
    let ips = vec!("127.0.0.1:3000");
    let mut res:Vec<HttpResult<Listening>> = vec!();
    
    for ip in ips {
        res.push(Iron::new(router()).http(ip));
    }
}
