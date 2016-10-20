use urlencoded::UrlEncodedBody;
use iron::prelude::*;

use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub text: String,
}

impl Config {
    /// New default config
    pub fn new(text: &str) -> Config {
        Config {
            text: text.to_string(),
         }
    }

    pub fn new_from_request(request: &mut Request)
                            -> Result<Config,String> {
        match request.get_ref::<UrlEncodedBody>() {
            Ok(hashmap) => {
                let config = match hashmap.get("text") {
                    Some(v) => Config::new(&v[0]),
                    None =>  return Err("Didn't find 'text' in POST hashmap".to_string())
                };
                Ok(config)
            },
            Err(ref e) => Err(e.description().to_string())
        }
    }
}
        
pub fn ips_from_args() -> Vec<String> {
    let mut args = env::args().into_iter();
    if args.len() < 2 {
        vec!("localhost:3000".to_string())
    } else {
        args.next();
        args.collect()
    }
}
   
