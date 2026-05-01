use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use threadpool::ThreadPool;

use ::response::Response;
use ::request::{Request};
use ::handler::Handler;
use ::headers::Headers;
use ::query::Query;
use ::parser::{Parser, ParserHandler, ParseError};

#[derive(Default)]
struct HttpParserHandler {
    method: String,
    url: String,
    query: Option<String>,
    version: String,
    headers: HashMap<String, Vec<String>>,
}

impl HttpParserHandler {
    pub fn build_request(&self, stream: &TcpStream) -> Request {
        let version_vec: Vec<&str> = self.version.split('.').collect();
        let http_version = (version_vec[0].parse().unwrap(), version_vec[1].parse().unwrap());
        let query = self.query.clone().map(|q| Query::from_str(&q));
        Request::new(
            &self.method,
            "http",
            &self.url,
            query,
            http_version,
            Headers::with_data(self.headers.clone()),
            None,
            stream,
        )
    }
}

impl ParserHandler for HttpParserHandler {
    fn on_method(&mut self, method: &str) -> Result<(), ParseError> {
        self.method = method.to_owned();
        Ok(())
    }

    fn on_url(&mut self, url: &str) -> Result<(), ParseError> {
        self.url = url.to_owned();
        Ok(())
    }

    fn on_query(&mut self, query: &str) -> Result<(), ParseError> {
        self.query = Some(query.to_owned());
        Ok(())
    }

    fn on_http_version(&mut self, version: &str) -> Result<(), ParseError> {
        self.version = version.to_owned();
        Ok(())
    }

    fn on_header(&mut self, field: &str, values: Vec<&str>) -> Result<(), ParseError> {
        self.headers.insert(field.to_owned(), values.into_iter().map(|val| val.to_owned()).collect());
        Ok(())
    }
}

/// Server that listen for connections on given address
///
/// The server will listen for connections on the given address,
/// create the request and response objects and pass them to the
/// handler to process the request
///
/// #Examples
///
/// ```
/// use std::env;
/// use http_server::HttpServer;
/// use http_server::handler::{ServerHandler, FileMode};
///
/// let root = env::home_dir().unwrap();
/// let handler = ServerHandler::<FileMode>::new(&root);
/// let server = HttpServer::new("127.0.0.1:9000", 4);
///
/// ```
#[allow(dead_code)]
pub struct HttpServer {
    addr: String,
    listener: TcpListener,
    threadpool: ThreadPool,
}

impl HttpServer {
    /// Creates a new instance of HttpServer
    pub fn new(addr: &str, num_threads: usize) -> HttpServer {
        let listener = TcpListener::bind(addr).ok().expect(format!("Could not bind to address {}", addr).as_ref());

        HttpServer {
            addr: addr.to_string(),
            listener: listener,
            threadpool: ThreadPool::new(num_threads),
        }
    }

    /// Start the server with the given handler
    ///
    /// When started, the server will block and listen for connections,
    /// creating the request and response and passing them to the handler
    /// when a client connects
    pub fn start(&self, handler: Box<Handler + Send + Sync>) {
        let arc = Arc::new(handler);
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let handler = arc.clone();

                    self.threadpool.execute(move || {
                        let mut http_parser = HttpParserHandler::default();

                        Parser::request(&mut http_parser).parse(&mut stream).unwrap_or_else(|e| {
                            println!("Erro parsing request: '{}'", e);
                        });

                        let mut request = http_parser.build_request(&stream);
                        let mut response = Response::from_stream(&stream).unwrap();

                        handler.handle_request(&mut request, &mut response).unwrap_or_else(|e| {
                            println!("Error handling request: '{}'", e);
                        });
                    });
                },
                Err(error) => println!("{:?}", error),
            }
        }
    }

    pub fn stop(&self) {
        drop(&self.listener);
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.stop();
    }
}
