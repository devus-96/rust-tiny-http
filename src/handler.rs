use std::any::Any;
use std::fs::{self, File, Metadata};
use std::io::{self, Write, ErrorKind};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::process::Command;

use conduit_mime_types;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

use crate::response::Response;
use crate::request::Request;

pub struct FileMode;
pub struct DirectoryMode;

pub trait Handler {
    fn handle_request(&self, req: &mut Request, res: &mut Response) -> Result<(), io::Error>;
}

#[derive(Debug)]
pub struct ServerHandler<M: Any> {
    root: PathBuf,
    _kind: PhantomData<M>,
}

impl<M: Any> ServerHandler<M> {
    pub fn new(root: &PathBuf) -> ServerHandler<M> {
        ServerHandler {
            root: root.to_owned(),
            _kind: PhantomData
        }
    }

    fn get_resource_and_metadata(&self, req: &Request) -> Result<(PathBuf, Metadata), io::Error> {
        let mut resource = Path::new(&self.root).to_path_buf();

        for p in req.path_components().iter() {
            resource = resource.join(p);
        }

        let metadata = fs::metadata(&resource)?;

        Ok((resource, metadata))
    }

    fn send_file(&self, resource: &Path, metadata: &Metadata, res: &mut Response) -> Result<(), io::Error> {
        let mut f = File::open(&resource)?;
        let mime = conduit_mime_types::mime_for_path(Path::new(&resource)).unwrap_or("application/octet-stream");

        res.with_header("Content-Type", mime)
            .with_header("Content-Length", &metadata.len().to_string());

        res.start(|res| {
            io::copy(&mut f, res)?;
            res.flush()?;
            Ok(())
        })
    }

    fn send_not_found(&self, res: &mut Response) -> Result<(), io::Error> {
        res.with_status(404, "Not Found");
        res.start(|res| {
            res.write("404 - Not Found".as_bytes())?;
            res.flush()?;
            Ok(())
        })
    }

    fn send_error(&self, res: &mut Response, status: i32, description: &str) -> Result<(), io::Error> {
        res.with_status(status, description);
        res.start(|res| {
            res.write(format!("{} - {}", status, description).as_bytes())?;
            res.flush()?;
            Ok(())
        })
    }
}

impl Handler for ServerHandler<FileMode> {
    fn handle_request(&self, req: &mut Request, res: &mut Response) -> Result<(), io::Error> {
        let (resource, metadata) = match self.get_resource_and_metadata(req) {
            Ok(result) => result,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return self.send_not_found(res);
                } else {
                    return self.send_error(res, 500, "Internal Server Error");
                }
            }
        };

        if !metadata.is_file() {
            return self.send_not_found(res);
        }

        self.send_file(&resource, &metadata, res)
    }
}

impl Handler for ServerHandler<DirectoryMode> {
    fn handle_request(&self, req: &mut Request, res: &mut Response) -> Result<(), io::Error> {
        let (resource, metadata) = match self.get_resource_and_metadata(req) {
            Ok(result) => result,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return self.send_not_found(res);
                } else {
                    return self.send_error(res, 500, "Internal Server Error");
                }
            }
        };

        if metadata.is_file() {
            return self.send_file(&resource, &metadata, res);
        }

        let output = Command::new("ls")
            .arg(&resource)
            .output()
            .unwrap_or_else(|e| panic!("Failed to list dir: {}", e));

        let s: String;
        if output.status.success() {
            s = String::from_utf8_lossy(&output.stdout).as_ref().to_owned();
        } else {
            s = String::from_utf8_lossy(&output.stderr).as_ref().to_owned();
            panic!("ls failed and stderr was:\n{}", s);
        }

        res.with_header("Content-Type", "text/html; charset=utf-8");

        res.start(|res| {
            res.write("<html><body><ul>".as_bytes())?;
            for name in s.split('\n') {
                if name.len() == 0 { continue }
                let mut name = name.to_owned();

                let metadata = fs::metadata(Path::new(&resource).join(&name))?;

                if metadata.is_dir() {
                    name = format!("{}/", name);
                }

                let mut path = req.path().to_owned();
                path.push_str(&name);
                let path = percent_encode(
                    path.as_bytes(),
                    NON_ALPHANUMERIC
                );

                res.write(format!("<li><a href=\"{0}\">{1}</a></li>", path, name).as_bytes())?;
            }
            res.write("</ul></body></html>".as_bytes())?;
            res.flush()?;

            Ok(())
        })
    }
}
