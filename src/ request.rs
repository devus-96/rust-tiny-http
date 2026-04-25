use std::io::Read;
use std::net::{SocketAddr, TcpStream};

use super::headers::Headers;
use super::query::Query;

#[allow(dead_code)]
pub struct Request {
    http_version: (u16, u16),
    method: String,
    scheme: String,
    path: Vec<String>,
    path_str: String,
    query: Option<Query>,
    headers: Headers,
    content_length: Option<u64>,
    stream: TcpStream,
}

impl Request {
    pub fn new(method: &str, scheme: &str, url: &str, query: Option<Query>,
               version: (u16, u16), headers: Headers,
               content_length: Option<u64>,
               stream: &TcpStream) -> Self {

       let path = url[1..url.len()].split('/').map(|x| x.to_owned()).collect();

       Request {
           http_version: version,
           method: method.to_owned(),
           scheme: scheme.to_owned(),
           path: path,
           path_str: url.to_owned(),
           headers: headers,
           query: query,
           content_length: content_length,
           stream: stream.try_clone().unwrap(),
       }
   }

    pub fn http_version(&self) -> (u16, u16) {
        self.http_version
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn host(&self) -> SocketAddr {
        self.stream.local_addr().unwrap()
    }

    pub fn path(&self) -> &str {
        &self.path_str
    }

    pub fn path_components(&self) -> Vec<&str> {
        self.path.iter().map(|i| i.as_ref()).collect()
    }

    pub fn query(&self) -> &Option<Query> {
        &self.query
    }

    pub fn remote_addr(&self) -> SocketAddr {
        self.stream.peer_addr().unwrap()
    }

    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body<'a>(&'a mut self) -> &'a mut Read {
        &mut self.stream
    }
}
