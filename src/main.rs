extern crate http_server;
extern crate argparse;

use std::env;
use argparse::{ArgumentParser, Store, StoreTrue};

use http_server::HttpServer;
use http_server::handler::{Handler, ServerHandler, FileMode, DirectoryMode};

const DEFAULT_ADDR: &'static str = "127.0.0.1:9000";

fn main() {
    let mut addr = DEFAULT_ADDR.to_owned();
    let mut dir_mode = false;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("tiny http server");
        parser.refer(&mut addr).add_option(&["-a", "--addr"], Store, "Address to listen");
        parser.refer(&mut dir_mode).add_option(&["-d", "--dir"], StoreTrue, "Enable directory listing within root");
        parser.parse_args_or_exit();
    }

    // Edit here to change the server root
    let path = env::home_dir().unwrap();

    let handler: Box<Handler + Send + Sync>;

    if dir_mode {
        handler = Box::new(ServerHandler::<DirectoryMode>::new(&path));
    } else {
        handler = Box::new(ServerHandler::<FileMode>::new(&path));
    }

    let server: HttpServer = HttpServer::new(&addr, 4usize);
    server.start(handler);
}
